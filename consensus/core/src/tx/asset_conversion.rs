use crate::asset_type::AssetType;
use serde::{Deserialize, Serialize};
use serde_cbor;

#[derive(Serialize, Deserialize)]
pub struct AssetConversionDetails {
    pub to_asset_type: AssetType,
    pub from_asset_type: AssetType,
    pub to_amount: u64,
    pub from_amount: u64,
}

pub struct AssetConversionSerializer;

impl AssetConversionSerializer {
    pub fn serialize(details: &AssetConversionDetails) -> Vec<u8> {
        serde_cbor::to_vec(details).expect("Serialization failed")
    }

    pub fn deserialize(data: &[u8]) -> AssetConversionDetails {
        serde_cbor::from_slice(data).expect("Deserialization failed")
    }
}
