use anchor_lang::prelude::*;
use ethprim::{address, uint, Address, U256};
use std::mem::size_of;

declare_id!("HswLWCKuBrz93fRxWNoU8yAsuUyvsntteXxDaWmU9KFy");

#[program]
pub mod ethprim_lib {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let from:Address = address!("");
        let amount:U256 = uint!("ff");
        msg!("send {} from {}",amount,from);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = signer,
        space=size_of::<Message>() + 8,
        seeds = [],
        bump)]
pub message: Account<'info, Message>,

#[account(mut)]
pub signer: Signer<'info>,

pub system_program: Program<'info, System>,
}
#[account]
pub struct Message {
    from: Address,
    amount:U256
 }
