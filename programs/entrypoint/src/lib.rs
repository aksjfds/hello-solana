#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
declare_id!("HrSp3iGMsXjgJXL5AZiA2UJ1XPaVDqaXgVUTGpZociiS");

#[program]
pub mod entrypoint {
    use anchor_lang::system_program::{self, Transfer};

    use super::*;

    pub fn start_auction(
        ctx: Context<StartAuction>,
        start_price: u64,
        end_price: u64,
        duration: u64,
        drop_interval: u64,
        drop_step: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let start_time = clock.unix_timestamp as u64;
        let end_time = start_time + duration;

        *ctx.accounts.auction_info = AuctionInfo {
            start_price,
            end_price,
            start_time,
            end_time,
            drop_interval,
            drop_step,
        };

        msg!("auction_info: {:?}", ctx.accounts.auction_info);
        Ok(())
    }

    pub fn auction(ctx: Context<AuctionTransfer>, amount: u64) -> Result<()> {
        // get current price
        let info = &ctx.accounts.auction_info;
        let current_time = Clock::get()?.unix_timestamp as u64;
        let steps = (current_time - info.start_time) / info.drop_interval;
        let current_price = info.start_price - (steps * info.drop_step);
        require!(amount > current_price, AuctionError::InsufficientSolError);

        // transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.pda.to_account_info(),
        };
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_context, amount)?;

        // log payer
        ctx.accounts.pda.payer = *ctx.accounts.payer.key;
        msg!("pda: {:?}", ctx.accounts.pda.payer);
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct AuctionPayer {
    payer: Pubkey,
}

#[derive(Accounts)]
pub struct AuctionTransfer<'info> {
    #[account(
        init,
        payer=payer,
        space=8 + AuctionPayer::INIT_SPACE,
        seeds=[b"pda"],
        bump)]
    pub pda: Account<'info, AuctionPayer>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds=[b"auction"], bump)]
    pub auction_info: Account<'info, AuctionInfo>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct AuctionInfo {
    pub start_price: u64,
    pub end_price: u64,

    pub start_time: u64,
    pub end_time: u64,

    pub drop_interval: u64,
    pub drop_step: u64,
}

#[derive(Accounts)]
pub struct StartAuction<'info> {
    #[account(
        init,
        payer=payer,
        space=8 + AuctionInfo::INIT_SPACE,
        seeds=[b"auction"],
        bump
    )]
    pub auction_info: Account<'info, AuctionInfo>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum AuctionError {
    #[msg("insufficient sol")]
    InsufficientSolError,
}
