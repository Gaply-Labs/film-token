use anchor_lang::prelude::*;

pub fn reveal_nft(ctx: Context<RevealNft>, params: RevealParams) -> Result<()> {
    let nft = &mut ctx.accounts.nft;

    if nft.revealed1 == false {
        if let (Some(name), Some(description), Some(image)) =
            (params.name, params.description, params.image)
        {
            msg!("Name: {}", name);
            msg!("Description: {}", description);
            msg!("Image: {}", image);
            nft.revealed1 = true;
        } else {
            return err!(crate::CustomErrorCode::Reveal1Error);
        }
    } else if nft.revealed1 == true && nft.revealed2 == false {
        if let Some(attributes) = params.attributes {
            msg!("Attributes: {:?}", attributes);
            nft.revealed2 = true;
        } else {
            return err!(crate::CustomErrorCode::Reveal2Error);
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct RevealNft<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub nft: Box<Account<'info, crate::Nft>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct RevealParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub attributes: Option<Vec<String>>,
}
