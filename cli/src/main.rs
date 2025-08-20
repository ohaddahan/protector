mod parse_validators;
use crate::parse_validators::ValidatorsHelper;

fn main() -> anyhow::Result<()> {
    // Open the file
    let validators_data = ValidatorsHelper::parse_validators("validators.json")?;
    let bad_blocks = ValidatorsHelper::count_bad_blocks(&validators_data, 0.15);
    println!("bad_blocks = {bad_blocks}");
    let bad_validators = ValidatorsHelper::filter_validators(validators_data, 0.15);
    println!("bad_validators => {bad_validators:?}");
    println!("bad_validators => {}", bad_validators.len());
    let mut groups = ValidatorsHelper::read_leader_schedule("leader-schedule.txt")?;
    ValidatorsHelper::keep_only_bad_validators(&bad_validators, &mut groups);
    let mut count = 0;
    for entry in groups {
        count += entry.1.len();
    }
    println!("count = {count}");
    Ok(())
}
