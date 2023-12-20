use kash_cli_lib::{kash_cli, TerminalOptions};

#[tokio::main]
async fn main() {
    let result = kash_cli(TerminalOptions::new().with_prompt("$ "), None).await;
    if let Err(err) = result {
        println!("{err}");
    }
}
