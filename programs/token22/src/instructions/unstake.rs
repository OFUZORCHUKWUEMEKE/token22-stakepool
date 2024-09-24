use {
    crate::{errors::*, state::*, utils::*}, anchor_lang::prelude::*, anchor_spl::{token::TransferChecked, token_interface}, solana_program::program::invoke_signed, spl_token_2022::instruction::{mint_to, transfer_checked}
};

pub fn handler(ctx:Context<Unstake>)->Result<()>{
    check_token_program(ctx.accounts.token_program.key());

    let user_entry = &ctx.accounts.user_stake_entry;
    let amount = user_entry.balance;
    let decimals = ctx.accounts.token_mint.decimals;

    msg!("User stake balance: {}", user_entry.balance);
    msg!("Withdrawing all of users stake balance. Tokens to withdraw: {}", amount);
    msg!("Total staked before withdrawal: {}", ctx.accounts.pool_state.amount);

    if amount > ctx.accounts.pool_state.amount{
        return Err(StakeError::OverdraeError.into())
    }
    // program signer seeds
    let auth_bump = ctx.accounts.pool_state.vault_auth_bump;
    let auth_seeds = &[VAULT_AUTH_SEED.as_bytes(), &[auth_bump]];
    let signer = &[&auth_seeds[..]];

      // transfer out_amount from stake vault to user
    let transfer_ix = transfer_checked(
        &ctx.accounts.token_program.key(),
        &ctx.accounts.token_vault.key(),
        &ctx.accounts.token_mint.key(),
        &ctx.accounts.user_token_account.key(),
        &ctx.accounts.pool_authority.key(),
        &[&ctx.accounts.pool_authority.key()],
        amount,
        6
    ).unwrap();
    invoke_signed(
        &transfer_ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.token_vault.to_account_info(),
            ctx.accounts.token_mint.to_account_info(),
            ctx.accounts.user_token_account.to_account_info(),
            ctx.accounts.pool_authority.to_account_info(),
        ],
        signer
    )?;

    // mint users staking rewards, 10x amount of staked tokens
    let stake_rewards = amount.checked_mul(10).unwrap();
 
    // mint rewards to user
    let mint_ix = mint_to(
        &ctx.accounts.token_program.key(),
        &ctx.accounts.staking_token_mint.key(),
        &ctx.accounts.user_stake_token_account.key(),
        &ctx.accounts.pool_authority.key(),
        &[&ctx.accounts.pool_authority.key()],
        stake_rewards
    ).unwrap();
    invoke_signed(
        &mint_ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.staking_token_mint.to_account_info(),
            ctx.accounts.user_stake_token_account.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.pool_authority.to_account_info(),
        ],
        signer
    )?;

    // borrow mutable references
    let user_entry = &mut ctx.accounts.user_stake_entry;
    let pool_state = &mut ctx.accounts.pool_state;
 
    msg!("Total staked after withdrawal: {}", pool_state.amount);
    // subtract transferred amount from pool total
    pool_state.amount = pool_state.amount.checked_sub(amount).unwrap();
    user_entry.balance = user_entry.balance.checked_sub(amount).unwrap();
 
    // update user stake entry
    user_entry.last_staked = Clock::get().unwrap().unix_timestamp;
    Ok(())

}


#[derive(Accounts)]
pub struct Unstake<'info> {
    // pool state account
    #[account(
        mut,
        seeds = [token_mint.key().as_ref(), STAKE_POOL_STATE_SEED.as_bytes()],
        bump = pool_state.bump,
    )]
    pub pool_state: Account<'info, PoolState>,
    // Mint of token
    #[account(
        mut,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds = [VAULT_AUTH_SEED.as_bytes()],
        bump
    )]
    pub pool_authority: UncheckedAccount<'info>,
    // pool token account for Token Mint
    #[account(
        mut,
        // use token_mint, pool auth, and constant as seeds for token a vault
        seeds = [token_mint.key().as_ref(), pool_authority.key().as_ref(), VAULT_SEED.as_bytes()],
        bump = pool_state.vault_bump,
        token::token_program = token_program
    )]
    pub token_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    // require a signature because only the user should be able to unstake their tokens
    #[account(
        mut,
        constraint = user.key() == user_stake_entry.user
        @ StakeError::InvalidUser
    )]
    pub user: Signer<'info>,
    #[account(
        mut,
        constraint = user_token_account.mint == pool_state.token_mint
        @ StakeError::InvalidMint,
        token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        seeds = [user.key().as_ref(), pool_state.token_mint.key().as_ref(), STAKE_ENTRY_SEED.as_bytes()],
        bump = user_stake_entry.bump,
 
    )]
    pub user_stake_entry: Account<'info, StakeEntry>,
    // Mint of staking token
    #[account(
        mut,
        mint::authority = pool_authority,
        mint::token_program = token_program,
        constraint = staking_token_mint.key() == pool_state.staking_token_mint
        @ StakeError::InvalidStakingTokenMint
    )]
    pub staking_token_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(
        mut,
        token::mint = staking_token_mint,
        token::authority = user,
        token::token_program = token_program,
        constraint = user_stake_token_account.key() == user_stake_entry.user_stake_token_account
        @ StakeError::InvalidUserStakeTokenAccount
    )]
    pub user_stake_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> Unstake<'info>{
    pub fn transfer_checked_ctx<'a>(&'a self, seeds: &'a [&[&[u8]]]) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from:self.token_vault.to_account_info(),
            to:self.user_token_account.to_account_info(),
            authority:self.pool_authority.to_account_info(),
            mint:self.token_mint.to_account_info(),
        };
        CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds)
        // transfer staked tokens
       
    }
 
    // mint_to
    pub fn mint_to_ctx<'a>(&'a self, seeds: &'a [&[&[u8]]]) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo{
            mint:self.staking_token_mint.to_account_info(),
            to:self.user_stake_token_account.to_account_info(),
            authority:self.pool_authority.to_account_info()
        };
        CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds)
    }
}