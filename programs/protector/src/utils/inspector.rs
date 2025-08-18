use crate::errors::Errors;
use anchor_lang::prelude::{AccountInfo, ProgramError, Pubkey};
use anchor_lang::solana_program::serialize_utils::read_u16;
use anchor_lang::solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked,
};
use anchor_lang::ToAccountInfo;

pub struct Inspector;

pub struct InstructionData {
    pub index: u16,
    pub my_index: u16,
    pub program_id: Pubkey,
}

impl Inspector {
    pub fn load_instructions<'a>(
        instruction_sysvar_account: &'a AccountInfo<'a>,
    ) -> Result<Vec<InstructionData>, Errors> {
        let mut instructions_data: Vec<InstructionData> = Vec::new();
        let data = instruction_sysvar_account
            .try_borrow_data()
            .map_err(|_| Errors::InspectorError1)?;
        let current_index =
            load_current_index_checked(instruction_sysvar_account.to_account_info().as_ref())
                .map_err(|_| Errors::InspectorError2)?;
        let mut current = 0;

        let num_instructions;
        match read_u16(&mut current, &**data) {
            Ok(index) => {
                num_instructions = index;
            }
            Err(_) => {
                return Err(Errors::InspectorError3);
            }
        }
        for index in 0..num_instructions {
            let instruction =
                load_instruction_at_checked(index as usize, &instruction_sysvar_account)
                    .map_err(|_| Errors::InspectorError4)?;
            instructions_data.push(InstructionData {
                program_id: instruction.program_id,
                my_index: current_index,
                index,
            });
        }
        Ok(instructions_data)
    }
}
