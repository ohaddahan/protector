use anchor_lang::prelude::*;

#[account]
pub struct LeaderSchedule {
    pub bump: u8,
}

impl LeaderSchedule {
    pub const SEED: &'static str = "LeaderSchedule";
    pub const SIZE: usize =
        // discriminator
        8;
}
