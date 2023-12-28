use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
/// Enum representing different types of assets in the Kash blockchain.
/// This allows for the representation of multiple currencies such as KSH, KUSD, and KRV.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[wasm_bindgen(js_name = assetType)]
pub enum AssetType {
    KSH = 0,
    KUSD = 1,
    KRV = 2,
}

impl From<u32> for AssetType {
    fn from(value: u32) -> Self {
        match value {
            0 => AssetType::KSH,
            1 => AssetType::KUSD,
            2 => AssetType::KRV,
            _ => panic!("Invalid AssetType value: {}", value),
        }
    }
}

impl From<AssetType> for u32 {
    fn from(val: AssetType) -> Self {
        match val {
            AssetType::KSH => 0,
            AssetType::KUSD => 1,
            AssetType::KRV => 2,
        }
    }
}

impl From<u8> for AssetType {
    fn from(value: u8) -> Self {
        match value {
            0 => AssetType::KSH,
            1 => AssetType::KUSD,
            2 => AssetType::KRV,
            _ => panic!("Invalid AssetType value: {}", value),
        }
    }
}

impl From<&str> for AssetType {
    fn from(value: &str) -> Self {
        match value {
            "KSH" => AssetType::KSH,
            "KUSD" => AssetType::KUSD,
            "KRV" => AssetType::KRV,
            _ => panic!("Invalid AssetType value: {}", value),
        }
    }
}

impl TryFrom<JsValue> for AssetType {
    type Error = JsValue;

    fn try_from(js_value: JsValue) -> Result<Self, Self::Error> {
        if let Some(value) = js_value.as_f64() {
            match value as u8 {
                0 => Ok(AssetType::KSH),
                1 => Ok(AssetType::KUSD),
                2 => Ok(AssetType::KRV),
                _ => Err(JsValue::from_str("Invalid AssetType value")),
            }
        } else {
            Err(JsValue::from_str("Invalid AssetType value"))
        }
    }
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssetType::KSH => "KSH",
                AssetType::KUSD => "KUSD",
                AssetType::KRV => "KRV",
            }
        )
    }
}
