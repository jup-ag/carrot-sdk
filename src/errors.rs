use thiserror::Error;

#[derive(Error, Clone, Copy, Debug)]
pub enum CarrotAmmError {
    #[error("Invalid Destination Mint")]
    InvalidDestinationMint = 0,

    #[error("Invalid Source Mint")]
    InvalidSourceMint = 1,

    #[error("Vault Shares State Not Initialized")]
    SharesStateNotInitialized = 2,

    #[error("Asset Not Found")]
    AssetNotFound = 3,

    #[error("Invalid Token Calculation")]
    InvalidTokenCalculation = 4,

    #[error("Insufficient Liquidity")]
    InsufficientLiquidity = 5,

    #[error("Invalid Fee Calculation")]
    InvalidFeeCalculation = 6,

    #[error("Oracle Price is Stale")]
    OraclePriceStale = 7,
}
