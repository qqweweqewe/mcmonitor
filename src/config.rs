use std::env;

pub struct Config {
    pub server_address: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    pub poll_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            server_address: env::var("MC_SERVER_ADDRESS")?,
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")?,
            telegram_chat_id: env::var("TELEGRAM_CHAT_ID")?,
            poll_interval_secs: env::var("POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
        })
    }
}
