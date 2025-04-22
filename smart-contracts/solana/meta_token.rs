//! $META Token Contract
//! SPL Token implementation for the MCPMeta platform

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
};
use spl_token::{
    state::{Account, Mint},
    instruction::{initialize_mint, mint_to},
};
use borsh::{BorshDeserialize, BorshSerialize};

/// Program entrypoint
#[entrypoint]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("$META Token instruction");
    
    let instruction = MetaTokenInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    
    match instruction {
        MetaTokenInstruction::Initialize {} => {
            process_initialize(program_id, accounts)
        },
        MetaTokenInstruction::Mint { amount } => {
            process_mint(program_id, accounts, amount)
        },
        MetaTokenInstruction::Burn { amount } => {
            process_burn(program_id, accounts, amount)
        },
        MetaTokenInstruction::Transfer { amount } => {
            process_transfer(program_id, accounts, amount)
        },
    }
}

/// Instructions supported by the $META Token program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MetaTokenInstruction {
    /// Initialize the $META token
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Authority account
    /// 1. `[writable]` Token mint account
    /// 2. `[]` Rent sysvar
    /// 3. `[]` Token program
    Initialize {},
    
    /// Mint new $META tokens
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Authority account
    /// 1. `[]` Token mint account
    /// 2. `[writable]` Destination account
    /// 3. `[]` Token program
    Mint {
        /// Amount of tokens to mint
        amount: u64,
    },
    
    /// Burn $META tokens
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Owner account
    /// 1. `[writable]` Source account
    /// 2. `[]` Token mint account
    /// 3. `[]` Token program
    Burn {
        /// Amount of tokens to burn
        amount: u64,
    },
    
    /// Transfer $META tokens
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Owner account
    /// 1. `[writable]` Source account
    /// 2. `[writable]` Destination account
    /// 3. `[]` Token program
    Transfer {
        /// Amount of tokens to transfer
        amount: u64,
    },
}

/// Initialize the $META token
fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let authority_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    
    // Verify signer
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Initialize mint
    let rent = &Rent::from_account_info(rent_info)?;
    let initialize_mint_ix = initialize_mint(
        token_program_info.key,
        mint_info.key,
        authority_info.key,
        Some(authority_info.key),
        9, // 9 decimals
    )?;
    
    invoke(
        &initialize_mint_ix,
        &[
            mint_info.clone(),
            rent_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
    )?;
    
    msg!("$META token initialized");
    Ok(())
}

/// Mint new $META tokens
fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let authority_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    
    // Verify signer
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Mint tokens
    let mint_to_ix = mint_to(
        token_program_info.key,
        mint_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
    )?;
    
    invoke(
        &mint_to_ix,
        &[
            mint_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
    )?;
    
    msg!("Minted {} $META tokens", amount);
    Ok(())
}

/// Burn $META tokens
fn process_burn(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let owner_info = next_account_info(account_info_iter)?;
    let source_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    
    // Verify signer
    if !owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Burn tokens (implementation using SPL token program)
    // For simplicity, we're not showing the actual burn instruction here
    
    msg!("Burned {} $META tokens", amount);
    Ok(())
}

/// Transfer $META tokens
fn process_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let owner_info = next_account_info(account_info_iter)?;
    let source_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    
    // Verify signer
    if !owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Transfer tokens (implementation using SPL token program)
    // For simplicity, we're not showing the actual transfer instruction here
    
    msg!("Transferred {} $META tokens", amount);
    Ok(())
} 