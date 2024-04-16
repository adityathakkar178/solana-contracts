use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ICOAccount {
    pub total_supply: u64,
    pub admin: Pubkey,
    pub balance: Vec<(Pubkey, u64)>,
    pub pre_sale_account: Vec<PreSaleAccount>,
    pub sale_account: Vec<SaleAccount>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PreSaleAccount {
    pub token_amount: u64,
    pub token_price: u64,
    pub whitelist_account: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SaleAccount {
    pub token_amount: u64,
    pub token_price: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("ICO Program Enter Point");

    let account_iter = &mut accounts.iter();
    let ico_accounts = next_account_info(account_iter)?;

    if ico_accounts.owner != program_id {
        msg!("ICO account does not have correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ico_state = ICOAccount::try_from_slice(&ico_accounts.data.borrow())?;

    match instruction_data[0] {
        0 => {
            intialize_ico(program_id, &mut ico_state, account_iter);
        }
        1 => {
            let recipient_account_info = next_account_info(account_iter)?;
            let amount_bytes = instruction_data[1..9].try_into().unwrap();
            let amount = u64::from_le_bytes(amount_bytes);
            mint_tokens(&mut ico_state, &recipient_account_info.key, amount)?;
        }
        2 => {
            pre_sale(&ico_state, accounts)?;
        }
        3 => {
            sale(&ico_state, accounts)?;
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    ico_state.serialize(&mut &mut ico_accounts.data.borrow_mut()[..])?;

    Ok(())
}

pub fn intialize_ico(
    program_id: &Pubkey,
    ico_state: &mut ICOAccount,
    account_iter: &mut std::slice::Iter<'_, AccountInfo>,
) -> ProgramResult {
    let admin_account = next_account_info(account_iter)?;

    if admin_account.key != program_id {
        msg!("Caller is not the admin");
        return Err(ProgramError::InvalidAccountData);
    }

    ico_state.admin = *admin_account.key;
    ico_state.total_supply = 10000;
    ico_state
        .balance
        .push((*admin_account.key, ico_state.total_supply));
    Ok(())
}

pub fn mint_tokens(
    ico_state: &mut ICOAccount,
    recipient_accounts: &Pubkey,
    amount: u64,
) -> ProgramResult {
    if let Some((_, balance)) = ico_state
        .balance
        .iter_mut()
        .find(|(account, _)| *account == *recipient_accounts)
    {
        *balance += amount;
        return Ok(());
    }
    ico_state.balance.push((*recipient_accounts, amount));

    Ok(())
}

pub fn pre_sale(ico_state: &ICOAccount, accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

pub fn sale(ico_state: &ICOAccount, accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}
