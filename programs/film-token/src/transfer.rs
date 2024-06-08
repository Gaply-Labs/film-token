// use anchor_lang::prelude::*;

// pub fn transfer(ctx: Context<Transfer>, to: Pubkey, price: u64) -> Result<()> {
//     let nft = &mut ctx.accounts.nft;
//     let authority = &mut ctx.accounts.authority;

//     // Check if the sender is the current owner
//     // if *authority.key != nft.owner {
//     //     return err!(crate::CustomErrorCode::NotOwner);
//     // }

//     // // Check if the recipient account exists
//     // if !ctx.accounts.to.is_initialized() {
//     //     return err!(crate::CustomErrorCode::RecipientNotExist);
//     // }

//     // let to_balance = ctx.accounts.to.lamports();
//     // if to_balance < price {
//     //     return err!(crate::CustomErrorCode::InsufficientBalance);
//     // }

//     // Transfer NFT ownership
//     // nft.owner = to;

//     // Transfer SOL to previous owner
//     // let ix = anchor_lang::solana_program::system_instruction::transfer(
//     //     &ctx.accounts.owner.key(),
//     //     &ctx.accounts.to.key(),
//     //     price,
//     // );

//     // anchor_lang::solana_program::program::invoke(
//     //     &ix,
//     //     &[
//     //         ctx.accounts.owner.to_account_info(),
//     //         ctx.accounts.to.to_account_info(),
//     //     ],
//     // )?;

//     Ok(())
// }

// #[derive(Accounts)]
// #[instruction(to: Pubkey, price: u64)]
// pub struct Transfer<'info> {
//     #[account(
//         mut,
//         // seeds = [crate::constant::NFT_SEED, authority.key().as_ref()],
//         // bump,
//         has_one = authority,
//     )]
//     pub nft: Box<Account<'info, crate::Nft>>,
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
