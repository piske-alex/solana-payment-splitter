use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

use crate::{instruction::EscrowInstruction, error::EscrowError, state::Escrow};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, program_id)
            }
            EscrowInstruction::Withdraw {} => {
                msg!("Instruction: InitWithdraw");
                Self::process_init_escrow(accounts, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let mut count = 0;
        let mut signers = Vec::new();
        while  count < 2 {
            let initializer = next_account_info(account_info_iter)?;
            if !initializer.is_signer{
                return Err(ProgramError::MissingRequiredSignature);
            }
            signers.push(initializer);
            count = count + 1;
        }
        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }
        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey_1 = *signers[0].key;
        escrow_info.initializer_pubkey_2 = *signers[1].key;

        Escrow::pack(escrow_info, &mut escrow_account.data.borrow_mut())?;
        
        Ok(())
    }

    fn withdraw(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let escrow_account = next_account_info(account_info_iter)?;
        let initializer_pubkey_1 =  next_account_info(account_info_iter)?;
        let initializer_pubkey_2 =  next_account_info(account_info_iter)?;
        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
        if initializer_pubkey_1.key != &escrow_info.initializer_pubkey_1{
            return Err(ProgramError::MissingRequiredSignature);
        }
        if initializer_pubkey_2.key != &escrow_info.initializer_pubkey_2{
            return Err(ProgramError::MissingRequiredSignature);
        }

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        
//
        **initializer_pubkey_1.lamports.borrow_mut() = initializer_pubkey_1.lamports()
        .checked_add((escrow_account.lamports()-rent.lamports_per_byte_year)/2)
        .ok_or(EscrowError::AmountOverflow)?;
        **escrow_account.lamports.borrow_mut() = escrow_account.lamports().checked_sub((escrow_account.lamports()-rent.lamports_per_byte_year)/2).ok_or(EscrowError::AmountOverflow)?;

        **initializer_pubkey_2.lamports.borrow_mut() = initializer_pubkey_2.lamports()
        .checked_add((escrow_account.lamports()-rent.lamports_per_byte_year)/2)
        .ok_or(EscrowError::AmountOverflow)?;
        **escrow_account.lamports.borrow_mut() = escrow_account.lamports().checked_sub((escrow_account.lamports()-rent.lamports_per_byte_year)/2).ok_or(EscrowError::AmountOverflow)?;
        Ok(())
    }
}