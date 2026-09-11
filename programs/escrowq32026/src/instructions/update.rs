use anchor_lang::prelude::*;

use crate::{Escrow, ESCROW_SEED};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}
impl<'info> Update<'info> {
    // transfer from taker_ata_b to maker_ata_b
    pub fn update(&mut self, receive_amount: u64) -> Result<()> {
        self.escrow.receive = receive_amount;
        Ok(())
    }
}
