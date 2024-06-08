use anchor_lang::prelude::*;

pub fn initialize(
    ctx: Context<Initialize>,
    start1: String,
    end1: String,
    start2: String,
    end2: String,
) -> Result<()> {
    let init = &mut ctx.accounts.init;
    // let authority_key = Pubkey::new(b"HzhqNUoHPjgnxgPEG1rDFZUrXMpbze1A2qqjtjEZLN9x");
    // if ctx.accounts.authority.key() != authority_key {
    //     return err!(crate::CustomErrorCode::Unauthorized);
    // }
    init.authority = ctx.accounts.authority.key();
    init.revealed1 = false;
    init.start1 = start1;
    init.end1 = end1;
    init.revealed2 = false;
    init.start2 = start2;
    init.end2 = end2;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Init>() + 128,
    )]
    pub init: Box<Account<'info, Init>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Init {
    pub authority: Pubkey,
    pub revealed1: bool,
    pub start1: String,
    pub end1: String,
    pub revealed2: bool,
    pub start2: String,
    pub end2: String,
}
