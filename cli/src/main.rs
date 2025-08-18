mod bloom;

use fastbloom_rs::{CountingBloomFilter, FilterBuilder, Membership};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn main() -> io::Result<()> {
    let mut builder = FilterBuilder::new(1_000, 0.001);
    println!("builder = {builder:#?}");
    let mut cbf: CountingBloomFilter = builder.build_counting_bloom_filter();
    cbf.add(b"4Lh7VjsEgG9XnShnq1CAsH37sWevuiWPfzqjWKo2s4DK");
    let array = cbf.get_u8_array();
    println!("array = {}", array.len());
    return Ok(());

    // Open the file
    let mut groups: HashMap<String, Vec<(u64, u64)>> = HashMap::new();
    let file = File::open("leader-schedule.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut prev_key: Option<String> = None;
    let mut prev_start: Option<u64> = None;
    let mut prev_end: Option<u64> = None;
    // Iterate over each line
    for line in reader.lines() {
        // Handle potential errors per line (e.g., invalid UTF-8)
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let trimmed = line.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let slot = parts[0].parse::<u64>().unwrap();
        let pubkey = parts[1];
        match &prev_key {
            Some(key) => {
                if key == pubkey {
                    prev_end = Some(slot);
                } else {
                    let pair = (prev_start.unwrap(), prev_end.unwrap());
                    let key = prev_key.clone().unwrap(); // Extract once to avoid repeated unwrap/clone
                    groups.entry(key).or_insert_with(Vec::new).push(pair);
                    prev_start = Some(slot);
                    prev_end = None;
                    prev_key = Some(pubkey.to_string());
                }
            }
            None => {
                prev_key = Some(pubkey.to_string());
                prev_start = Some(slot);
                prev_end = None;
            }
        }
    }
    println!("groups => {groups:#?}");
    let mut count = 0;
    for entry in groups {
        count += entry.1.len();
    }
    println!("count = {count}");
    Ok(())
}
