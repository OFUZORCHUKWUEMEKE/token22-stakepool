use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError{
    #[msg("Token mint is invalid")]
    InvalidMint,
    #[msg("Mint Authority is invalid")]
    InvalidMintAuthority,
    #[msg("Mathematical Overflow Occurred")]
    MatheMaticalOverFlowError,
    #[msg("Incorrrect Program Authority")]
    InvalidProgramAuthority,
    #[msg("Attempted to withdraw more staking reqards than are available")]
    OverdraeError,
    #[msg("Invalid user Provided")]
    InvalidUser,
    #[msg("Invalid Staking token mint Provider")]
    InvalidStakingTokenMint,
    #[msg("Given user stake token account does not match what is stored in user stake entry!")]
    InvalidUserStakeTokenAccount
}