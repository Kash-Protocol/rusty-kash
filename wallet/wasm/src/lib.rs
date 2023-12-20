use kash_cli_lib::kash_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_kash_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    kash_cli(options, None).await?;
    Ok(())
}
