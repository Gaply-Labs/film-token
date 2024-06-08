use anchor_lang::prelude::*;

pub fn reveal(ctx: Context<Reveal>) -> Result<()> {
    let init = &mut ctx.accounts.init;

    if init.revealed1 == false {
        init.revealed1 = true;
    } else if init.revealed1 == true && init.revealed2 == false {
        init.revealed2 = true;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Reveal<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub init: Box<Account<'info, crate::Init>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
