use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AssetTypeError {
    #[error("asset type {0} is not supported")]
    UnsupportedAssetType(u8),
}
