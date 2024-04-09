use std::error::Error;
use std::fs::{File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    token: String,
    intents: u32
}

mod discord {
    pub mod client;
}

use discord::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json_file_path: &Path = Path::new("./config.json");
    let file: File = File::open(json_file_path)?;
    let config: Config = serde_json::from_reader(file).expect("Error while reading JSON file.");

    let mut discord_client: Client = Client::new(config.token, config.intents);
    discord_client.login().await.expect("Error logging into discord.");
    Ok(())
}