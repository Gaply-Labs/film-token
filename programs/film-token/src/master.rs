use anchor_lang::prelude::*;

pub fn master(ctx: Context<Master>) -> Result<()> {
    let state = &mut ctx.accounts.state;

    state.count = 0;
    state.nfts = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88".parse().unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Master<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<State>() + 256,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct State {
    pub count: u8,
    pub nfts: String,
}
