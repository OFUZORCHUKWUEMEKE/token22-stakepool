use {
    anchor_lang::prelude::*,
    crate::{state::*, errors::*, utils::*},
    std::mem::size_of,
    anchor_spl::{
        token_interface,
        associated_token::AssociatedToken,
    },
};


#[derive(Accounts)]
pub struct IniializeStakeEntry<'info>{
    
}