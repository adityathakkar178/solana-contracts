use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

use crate::instructions::{
    create::{create_token, CreateTokenArgs},
    mint::{mint_token, MintSplArgs},
    transfer::{transfer_tokens, TrasnferTokensArgs},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum MyInstruction {
    Create(CreateTokenArgs),
    MintSpl(MintSplArgs),
    TransferTokens(TrasnferTokensArgs),
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Create(args) => create_token(accounts, args),
        MyInstruction::MintSpl(args) => mint_token(accounts, args),
        MyInstruction::TransferTokens(args) => transfer_tokens(accounts, args),
    }
}
