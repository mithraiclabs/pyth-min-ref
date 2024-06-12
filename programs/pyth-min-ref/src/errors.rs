use anchor_lang::error_code;
use pyth_min::error::GetPriceError;

#[error_code]
pub enum ErrorCode {
    #[msg("Price too old")]
    PriceTooOld, // 6000
    #[msg("Mismatched Feed Id")]
    MismatchedFeedId, // 6001
    #[msg("Insuff Verification level")]
    InsufficientVerificationLevel, // 6002
    #[msg("Feed not 32 bytes")]
    FeedIdMustBe32Bytes, // 6003
    #[msg("Feed not in hex")]
    FeedIdNonHexCharacter, // 6004
}

impl From<GetPriceError> for ErrorCode {
    fn from(error: GetPriceError) -> Self {
        match error {
            GetPriceError::PriceTooOld => ErrorCode::PriceTooOld,
            GetPriceError::MismatchedFeedId => ErrorCode::MismatchedFeedId,
            GetPriceError::InsufficientVerificationLevel => {
                ErrorCode::InsufficientVerificationLevel
            }
            GetPriceError::FeedIdMustBe32Bytes => ErrorCode::FeedIdMustBe32Bytes,
            GetPriceError::FeedIdNonHexCharacter => ErrorCode::FeedIdNonHexCharacter,
        }
    }
}