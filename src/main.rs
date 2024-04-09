use std::error::Error;

mod discord {
    pub mod client;
}

use discord::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    Ok(())
}