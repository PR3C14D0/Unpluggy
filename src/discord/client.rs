pub struct Client {
    token: String,
    intents: u32
}


impl Client {
    pub fn new(token: String, intents: u32) -> Client {
        Client { token, intents }
    }
}
