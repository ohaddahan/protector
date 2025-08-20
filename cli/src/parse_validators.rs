use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ValidatorsHelper;

pub type ValidatorSchedule = HashMap<String, Vec<(u64, u64)>>;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "blocksProduced")]
    pub blocks_produced: u64,
    #[serde(rename = "blocksWithSandwiches")]
    pub blocks_with_sandwiches: u64,
    #[serde(rename = "sandwichRate")]
    pub sandwich_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorStat {
    #[serde(rename = "voteAccount")]
    pub vote_account: String,
    #[serde(rename = "identityAccount")]
    pub identity_account: String,
    #[serde(rename = "activeStake")]
    pub active_stake: u64,
    #[serde(rename = "activeStakeDisplay")]
    pub active_stake_display: String,
    pub commission: f64,
    pub stats30d: Option<Stats>,
    pub stats60d: Option<Stats>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorsData {
    ok: bool,
    err: Option<String>,
    data: Vec<ValidatorStat>,
}

impl ValidatorsHelper {
    pub fn parse_validators(path: &str) -> anyhow::Result<Vec<ValidatorStat>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let json: ValidatorsData = serde_json::from_reader(reader)?;
        Ok(json.data)
    }

    pub fn filter_validators(validators_data: Vec<ValidatorStat>, rate: f64) -> HashSet<String> {
        let validators: Vec<String> = validators_data
            .into_iter()
            .filter_map(|i| match i.stats30d {
                Some(stats30d) => {
                    let sandwich_rate =
                        stats30d.blocks_with_sandwiches as f64 / stats30d.blocks_produced as f64;
                    if sandwich_rate >= rate {
                        Some(i.vote_account)
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect();
        let validators: HashSet<String> = HashSet::from_iter(validators);
        validators
    }

    pub fn read_leader_schedule(path: &str) -> anyhow::Result<ValidatorSchedule> {
        let mut groups: ValidatorSchedule = HashMap::new();
        let file = File::open(path)?;
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
            let slot = parts[0].parse::<u64>()?;
            let pubkey = parts[1];
            match &prev_key {
                Some(key) => {
                    if key == pubkey {
                        prev_end = Some(slot);
                    } else {
                        let pair = (prev_start.unwrap(), prev_end.unwrap());
                        let key = prev_key.clone().unwrap(); // Extract once to avoid repeated unwrap/clone
                        groups.entry(key).or_default().push(pair);
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
        let pair = (
            prev_start.unwrap(),
            prev_end.unwrap_or(prev_start.unwrap() + 1),
        );
        let key = prev_key.clone().unwrap(); // Extract once to avoid repeated unwrap/clone
        match groups.get(&key) {
            Some(vec) => {
                if vec.iter().find(|i| i.0 == pair.0).is_none() {
                    groups.entry(key).or_default().push(pair);
                }
            }
            None => {
                groups.entry(key).or_default().push(pair);
            }
        }
        Ok(groups)
    }

    pub fn keep_only_bad_validators(
        bad_validators: &HashSet<String>,
        groups: &mut ValidatorSchedule,
    ) {
        groups.retain(|k, _| bad_validators.contains(k));
    }

    pub fn count_bad_blocks(validators_data: &Vec<ValidatorStat>, rate: f64) -> u64 {
        let mut count = 0;
        validators_data.iter().for_each(|i| {
            if let Some(stats30d) = &i.stats30d {
                let sandwich_rate =
                    stats30d.blocks_with_sandwiches as f64 / stats30d.blocks_produced as f64;
                if sandwich_rate >= rate {
                    count += stats30d.blocks_with_sandwiches;
                }
            }
        });
        count
    }
}
