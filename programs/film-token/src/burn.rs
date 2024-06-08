use anchor_lang::prelude::*;

pub fn burn(
    ctx: Context<Burn>,
    phone: String,
    name: String,
    email: String,
    metadata: String,
) -> Result<()> {
    let burn = &mut ctx.accounts.burn;
    let nft = &mut ctx.accounts.nft;

    if nft.burned {
        return err!(crate::CustomErrorCode::AlreadyUsed);
    }

    nft.burned = true;

    burn.phone = phone;
    burn.name = name;
    burn.email = email;
    burn.metadata = metadata;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Burn<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub nft: Box<Account<'info, crate::Nft>>,
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Burns>(),
    )]
    pub burn: Box<Account<'info, Burns>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Burns {
    pub authority: Pubkey,
    pub phone: String,
    pub name: String,
    pub email: String,
    pub metadata: String,
}
