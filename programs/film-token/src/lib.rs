use anchor_lang::prelude::*;

pub mod burn;
pub mod error;
pub mod initialize;
pub mod master;
pub mod mint;
pub mod reveal;
pub mod reveal_nft;

use error::*;
use {burn::*, initialize::*, master::*, mint::*, reveal::*, reveal_nft::*};

declare_id!("4rBgijn99e82bEF3n8uvYZsCkmng6BisrmFi13uCnb9c");

#[program]
mod film_token {
    use super::*;

    pub fn master(ctx: Context<Master>) -> Result<()> {
        master::master(ctx)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        start1: String,
        end1: String,
        start2: String,
        end2: String,
    ) -> Result<()> {
        initialize::initialize(ctx, start1, end1, start2, end2)
    }

    pub fn mint(ctx: Context<Mint>) -> Result<()> {
        mint::mint(ctx)
    }

    pub fn burn(
        ctx: Context<Burn>,
        phone: String,
        name: String,
        email: String,
        metadata: String,
    ) -> Result<()> {
        burn::burn(ctx, phone, name, email, metadata)
    }

    pub fn reveal(ctx: Context<Reveal>) -> Result<()> {
        reveal::reveal(ctx)
    }

    pub fn reveal_nft(ctx: Context<RevealNft>, params: RevealParams) -> Result<()> {
        reveal_nft::reveal_nft(ctx, params)
    }
}
