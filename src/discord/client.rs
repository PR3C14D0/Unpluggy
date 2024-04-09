use std::error::Error;
use serde::{Deserialize};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};

#[derive()]
pub struct Client {
    pub token: String,
    pub intents: u32,
    client: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[derive(Deserialize)]
struct Gateway {
    url: String
}


impl Client {
    pub fn new(token: String, intents: u32) -> Self {
        Client { token, intents, client: None }
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let res: Gateway = reqwest::get("https://discord.com/api/v10/gateway")
            .await?
            .json::<Gateway>()
            .await?;

        let (socket, _) = connect_async(res.url).await.expect("Failed to connect to websocket");
        self.client = Some(socket);

        println!("Handshake between client and websocket done");

        Ok(())
    }
}
