use crate::errors::Errors;
use anchor_lang::prelude::*;
use bloom_sol::bloom_sol::struct_def::CountingBloomFilter;

#[account]
pub struct BadPrograms {
    pub bump: u8,
    pub filter: Vec<u8>,
}

impl BadPrograms {
    pub const SEED: &'static str = "BadPrograms";
    pub const SIZE: usize =
        // discriminator
        8 +
        // filter
        8000;

    pub fn get_bloom_filter(&self) -> Result<CountingBloomFilter> {
        Ok(if self.filter.is_empty() {
            CountingBloomFilter::new(7_000, 5, true)
        } else {
            CountingBloomFilter::deserialize(&self.filter)
                .map_err(|_| Errors::DeserializeFilterFailure)?
        })
    }

    pub fn add_to_bloom_filter(value: &Pubkey, filter: &mut CountingBloomFilter) {
        filter.insert(value.as_array())
    }

    pub fn pre_save(&mut self, filter: &CountingBloomFilter) {
        self.filter = filter.serialize()
    }
}
