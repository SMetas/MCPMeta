//! MCP Game Distribution Platform Marketplace Contract
//! Handles MCP module trading and revenue distribution

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program::{invoke, invoke_signed},
};
use borsh::{BorshDeserialize, BorshSerialize};

/// Program entrypoint
#[entrypoint]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("MCP Marketplace instruction");
    
    let instruction = MarketplaceInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    
    match instruction {
        MarketplaceInstruction::InitializeMarketplace { fee_percentage } => {
            process_initialize_marketplace(program_id, accounts, fee_percentage)
        },
        MarketplaceInstruction::ListModule { price, is_free_issuance } => {
            process_list_module(program_id, accounts, price, is_free_issuance)
        },
        MarketplaceInstruction::PurchaseModule {} => {
            process_purchase_module(program_id, accounts)
        },
        MarketplaceInstruction::CollectRevenue {} => {
            process_collect_revenue(program_id, accounts)
        },
    }
}

/// Instructions supported by the Marketplace program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MarketplaceInstruction {
    /// Initialize a new marketplace
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Authority account
    /// 1. `[writable]` Marketplace account
    /// 2. `[]` Rent sysvar
    InitializeMarketplace {
        /// Platform fee percentage (e.g., 10 for 10%)
        fee_percentage: u8,
    },
    
    /// List an MCP module for sale
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Creator account
    /// 1. `[]` Marketplace account
    /// 2. `[writable]` Module account
    /// 3. `[]` Module token mint
    ListModule {
        /// Price in $META tokens
        price: u64,
        /// Whether this is free issuance (tips only)
        is_free_issuance: bool,
    },
    
    /// Purchase an MCP module
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Buyer account
    /// 1. `[]` Marketplace account
    /// 2. `[writable]` Module account
    /// 3. `[writable]` Creator account
    /// 4. `[writable]` Platform account
    /// 5. `[writable]` Buyer token account
    /// 6. `[writable]` Module token mint
    PurchaseModule {},
    
    /// Collect revenue from module sales
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Creator account
    /// 1. `[]` Marketplace account
    /// 2. `[writable]` Creator revenue account
    CollectRevenue {},
}

/// Marketplace account state
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Marketplace {
    /// Is the marketplace initialized
    pub is_initialized: bool,
    /// Authority pubkey
    pub authority: Pubkey,
    /// Platform fee percentage (0-100)
    pub fee_percentage: u8,
    /// Total revenue generated
    pub total_revenue: u64,
    /// Total modules sold
    pub total_modules_sold: u64,
}

impl IsInitialized for Marketplace {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

/// Module account state
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Module {
    /// Is the module initialized
    pub is_initialized: bool,
    /// Creator pubkey
    pub creator: Pubkey,
    /// Module price in $META tokens
    pub price: u64,
    /// Is free issuance (tips only)
    pub is_free_issuance: bool,
    /// Module URI (IPFS CID)
    pub uri: String,
    /// Total sales
    pub total_sales: u64,
    /// Total revenue
    pub total_revenue: u64,
}

impl IsInitialized for Module {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

/// Initialize a new marketplace
fn process_initialize_marketplace(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee_percentage: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let authority_info = next_account_info(account_info_iter)?;
    let marketplace_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    
    // Check ownership
    if marketplace_info.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Verify signer
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Verify fee percentage
    if fee_percentage > 100 {
        return Err(ProgramError::InvalidArgument);
    }
    
    // Check rent exemption
    let rent = &Rent::from_account_info(rent_info)?;
    if !rent.is_exempt(marketplace_info.lamports(), marketplace_info.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }
    
    // Initialize marketplace state
    let marketplace = Marketplace {
        is_initialized: true,
        authority: *authority_info.key,
        fee_percentage,
        total_revenue: 0,
        total_modules_sold: 0,
    };
    
    marketplace.serialize(&mut *marketplace_info.data.borrow_mut())?;
    
    msg!("Marketplace initialized with {}% fee", fee_percentage);
    Ok(())
}

/// List an MCP module for sale
fn process_list_module(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    price: u64,
    is_free_issuance: bool,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let creator_info = next_account_info(account_info_iter)?;
    let marketplace_info = next_account_info(account_info_iter)?;
    let module_info = next_account_info(account_info_iter)?;
    let module_mint_info = next_account_info(account_info_iter)?;
    
    // Verify signer
    if !creator_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Check ownership
    if module_info.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Check marketplace
    let marketplace = Marketplace::try_from_slice(&marketplace_info.data.borrow())?;
    if !marketplace.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }
    
    // If not free issuance, verify price
    if !is_free_issuance && price == 0 {
        return Err(ProgramError::InvalidArgument);
    }
    
    // Initialize module state
    let module = Module {
        is_initialized: true,
        creator: *creator_info.key,
        price,
        is_free_issuance,
        uri: String::new(), // Will be set separately
        total_sales: 0,
        total_revenue: 0,
    };
    
    module.serialize(&mut *module_info.data.borrow_mut())?;
    
    msg!("Module listed for sale at {} $META", price);
    Ok(())
}

/// Purchase an MCP module
fn process_purchase_module(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let buyer_info = next_account_info(account_info_iter)?;
    let marketplace_info = next_account_info(account_info_iter)?;
    let module_info = next_account_info(account_info_iter)?;
    let creator_info = next_account_info(account_info_iter)?;
    let platform_info = next_account_info(account_info_iter)?;
    let buyer_token_info = next_account_info(account_info_iter)?;
    let module_mint_info = next_account_info(account_info_iter)?;
    
    // Verify signer
    if !buyer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Load marketplace and module
    let mut marketplace = Marketplace::try_from_slice(&marketplace_info.data.borrow())?;
    if !marketplace.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }
    
    let mut module = Module::try_from_slice(&module_info.data.borrow())?;
    if !module.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }
    
    // Calculate fees (90% to creator, 10% to platform)
    let platform_fee = (module.price * marketplace.fee_percentage as u64) / 100;
    let creator_amount = module.price - platform_fee;
    
    // Process payment (simplified - would use token program in production)
    // This is a placeholder for the actual token transfer logic
    msg!("Processing payment of {} $META", module.price);
    msg!("Creator receives: {} $META", creator_amount);
    msg!("Platform fee: {} $META", platform_fee);
    
    // Update state
    module.total_sales += 1;
    module.total_revenue += module.price;
    marketplace.total_revenue += module.price;
    marketplace.total_modules_sold += 1;
    
    // Save updated state
    module.serialize(&mut *module_info.data.borrow_mut())?;
    marketplace.serialize(&mut *marketplace_info.data.borrow_mut())?;
    
    msg!("Module purchased successfully");
    Ok(())
}

/// Collect revenue from module sales
fn process_collect_revenue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let creator_info = next_account_info(account_info_iter)?;
    let marketplace_info = next_account_info(account_info_iter)?;
    let creator_revenue_info = next_account_info(account_info_iter)?;
    
    // Verify signer
    if !creator_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Simplified implementation - in production, would track and transfer
    // actual revenue amounts
    
    msg!("Revenue collected by creator");
    Ok(())
} 