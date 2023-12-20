use crate::result::Result;
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen(js_name = "sompiToKash")]
pub fn sompi_to_kash(sompi: JsValue) -> Result<f64> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_kash(sompi))
}

#[wasm_bindgen(js_name = "kashToSompi")]
pub fn kash_to_sompi(kash: f64) -> u64 {
    crate::utils::kash_to_sompi(kash)
}

#[wasm_bindgen(js_name = "sompiToKashString")]
pub fn sompi_to_kash_string(sompi: JsValue) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_kash_string(sompi))
}

#[wasm_bindgen(js_name = "sompiToKashStringWithSuffix")]
pub fn sompi_to_kash_string_with_suffix(sompi: JsValue, wallet: &crate::wasm::wallet::Wallet) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    let network_type = wallet.wallet.network_id()?.network_type;
    Ok(crate::utils::sompi_to_kash_string_with_suffix(sompi, &network_type))
}
