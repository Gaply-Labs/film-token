use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("You are not the owner of this NFT")]
    NotOwner,
    #[msg("Only the contract authority can call this function")]
    Unauthorized,
    #[msg("Maximum number of NFTs reached")]
    MaxNFTsReached,
    #[msg("Incorrect price for transferring NFT")]
    IncorrectPrice,
    #[msg("Insufficient payment for minting NFT")]
    InsufficientPayment,
    #[msg("Recipient account does not exist")]
    RecipientNotExist,
    #[msg("Insufficient balance in recipient account")]
    InsufficientBalance,
    #[msg("No price specified")]
    NoPriceSpecified,
    #[msg("Already used")]
    AlreadyUsed,
    #[msg("Already Revealed")]
    AlreadyRevealed,
    #[msg("End date must be after start date")]
    DateError,
    #[msg("Name, description, and image must be provided.")]
    Reveal1Error,
    #[msg("Attributes must be provided.")]
    Reveal2Error,
}
