use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A simple Counting Bloom Filter implementation in Rust.
///
/// This allows for insertion, deletion, and membership queries with possible false positives.
/// Counters are u32 to handle multiple insertions of the same item.
pub struct CountingBloomFilter {
    counters: Vec<u32>,
    size: usize,
    num_hashes: usize,
}

impl CountingBloomFilter {
    /// Creates a new CountingBloomFilter.
    ///
    /// - `size`: The number of counters (should be chosen based on expected number of elements and false positive rate).
    /// - `num_hashes`: The number of hash functions to use (typically 3-7 for good performance).
    pub fn new(size: usize, num_hashes: usize) -> Self {
        CountingBloomFilter {
            counters: vec![0; size],
            size,
            num_hashes,
        }
    }

    /// Computes the hash indices for a given item.
    fn get_hash_indices<T: Hash>(&self, item: &T) -> Vec<usize> {
        (0..self.num_hashes)
            .map(|i| {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                // Seed with the hash index to simulate multiple hash functions.
                hasher.write_u32(i as u32);
                (hasher.finish() % self.size as u64) as usize
            })
            .collect()
    }

    /// Adds an item to the filter.
    pub fn insert<T: Hash>(&mut self, item: &T) {
        for index in self.get_hash_indices(item) {
            self.counters[index] = self.counters[index].saturating_add(1);
        }
    }

    /// Removes an item from the filter if it exists.
    ///
    /// Note: Due to the probabilistic nature, removing non-inserted items may affect accuracy.
    pub fn remove<T: Hash>(&mut self, item: &T) {
        for index in self.get_hash_indices(item) {
            if self.counters[index] > 0 {
                self.counters[index] -= 1;
            }
        }
    }

    /// Checks if an item might be in the filter.
    ///
    /// Returns `true` if the item is possibly present (may be false positive), `false` if definitely not present.
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        self.get_hash_indices(item)
            .iter()
            .all(|&index| self.counters[index] > 0)
    }

    /// Estimates the current false positive probability (error rate) based on the fill level.
    ///
    /// This uses the standard Bloom filter approximation: (1 - e^(-k * n / m))^k,
    /// where n is estimated as (sum of counters) / k.
    /// Note: This is an approximation and assumes no counter overflows or excessive deletions.
    pub fn estimated_false_positive_rate(&self) -> f64 {
        let sum: u64 = self.counters.iter().map(|&c| c as u64).sum();
        let n_eff = sum as f64 / self.num_hashes as f64;
        let m = self.size as f64;
        let k = self.num_hashes as f64;

        if m == 0.0 {
            return 1.0;
        }

        let exp_term = (-k * n_eff / m).exp();
        (1.0 - exp_term).powf(k)
    }

    /// Serializes the bloom filter to a Vec<u8> in little-endian format.
    ///
    /// Format: [size (usize as 8 bytes LE)] + [num_hashes (usize as 8 bytes LE)] + [counters (u32 LE each)]
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(16 + self.size * 4);
        buf.extend_from_slice(&self.size.to_le_bytes());
        buf.extend_from_slice(&self.num_hashes.to_le_bytes());
        for &count in &self.counters {
            buf.extend_from_slice(&count.to_le_bytes());
        }
        buf
    }

    /// Deserializes a Vec<u8> back into a CountingBloomFilter.
    ///
    /// Returns an error if the data is invalid (wrong length, etc.).
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        const USIZE_BYTES: usize = std::mem::size_of::<usize>();
        if data.len() < 2 * USIZE_BYTES {
            return Err("Data too short for size and num_hashes");
        }

        let size_bytes: [u8; 8] = data[0..8].try_into().map_err(|_| "Invalid size bytes")?;
        let size = usize::from_le_bytes(size_bytes);

        let num_hashes_bytes: [u8; 8] = data[8..16]
            .try_into()
            .map_err(|_| "Invalid num_hashes bytes")?;
        let num_hashes = usize::from_le_bytes(num_hashes_bytes);

        let expected_len = 16 + size * 4;
        if data.len() != expected_len {
            return Err("Data length does not match expected size");
        }

        let mut counters = vec![0u32; size];
        for i in 0..size {
            let start = 16 + i * 4;
            let count_bytes: [u8; 4] = data[start..start + 4]
                .try_into()
                .map_err(|_| "Invalid counter bytes")?;
            counters[i] = u32::from_le_bytes(count_bytes);
        }

        Ok(Self {
            counters,
            size,
            num_hashes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_bloom_filter() {
        let mut cbf = CountingBloomFilter::new(1000, 5);

        cbf.insert(&"apple");
        cbf.insert(&"banana");
        cbf.insert(&"apple"); // Insert twice

        assert!(cbf.contains(&"apple"));
        assert!(cbf.contains(&"banana"));
        assert!(!cbf.contains(&"cherry"));

        cbf.remove(&"apple");
        assert!(cbf.contains(&"apple")); // Still present due to second insert

        cbf.remove(&"apple");
        assert!(!cbf.contains(&"apple")); // Now removed

        cbf.remove(&"banana");
        assert!(!cbf.contains(&"banana"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut cbf = CountingBloomFilter::new(1000, 5);
        cbf.insert(&"apple");
        cbf.insert(&"banana");

        let serialized = cbf.serialize();
        let deserialized = CountingBloomFilter::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.size, cbf.size);
        assert_eq!(deserialized.num_hashes, cbf.num_hashes);
        assert_eq!(deserialized.counters, cbf.counters);

        assert!(deserialized.contains(&"apple"));
        assert!(deserialized.contains(&"banana"));

        // Test invalid data
        assert!(CountingBloomFilter::deserialize(&[]).is_err());
        assert!(CountingBloomFilter::deserialize(&[0u8; 15]).is_err());
        let mut invalid = serialized.clone();
        invalid.truncate(invalid.len() - 1); // Shorten by one byte
        assert!(CountingBloomFilter::deserialize(&invalid).is_err());
    }
}
