use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint as iMint, MintTo, Token, TokenAccount},
};

pub fn mint(ctx: Context<Mint>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let signer = &mut ctx.accounts.authority;
    let nft = &mut ctx.accounts.nft;

    if state.count == 88 {
        return err!(crate::CustomErrorCode::MaxNFTsReached);
    }

    // let wallet = "9CABN6GAa2WBP2D6vqxMXuv226As1tkYWTyRSUgEJQam";

    // invoke(
    //     &transfer(
    //         &signer.key(),
    //         &Pubkey::from_str(wallet).unwrap(),
    //         70_000_000,
    //     ),
    //     &[
    //         signer.to_account_info(),
    //         // init.to_account_info(),
    //         ctx.accounts.system_program.to_account_info(),
    //     ],
    // )?;

    // Increase total todo count
    state.count = state.count.checked_add(1).unwrap();

    nft.authority = signer.key();
    nft.burned = false;
    nft.revealed1 = false;
    nft.revealed2 = false;

    let numbers_string = state.nfts.clone();
    let mut numbers: Vec<&str> = numbers_string.split(',').collect();

    let random_index = {
        let ptr_value = numbers_string.as_ptr() as usize;
        let index = (ptr_value ^ (ptr_value >> 4) ^ (ptr_value >> 8)) % numbers.len();
        index
    };

    let removed_number_str = numbers.remove(random_index);

    state.nfts = numbers.join(",");
    nft.metadata = removed_number_str.parse().unwrap();

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.imint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Mint<'info> {
    #[account(mut)]
    pub state: Box<Account<'info, crate::State>>,

    #[account(
        init,
        payer = authority,
        space = std::mem::size_of::<Nft>() + 8,
    )]
    pub nft: Box<Account<'info, Nft>>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
    )]
    pub imint: Account<'info, iMint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = imint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
#[derive(Default)]
pub struct Nft {
    pub authority: Pubkey,
    pub metadata: u8,
    pub revealed1: bool,
    pub revealed2: bool,
    pub burned: bool,
}
