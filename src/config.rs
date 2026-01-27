use std::env;

pub struct Config {
    pub server_address: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    pub log_topic_id: i32,
    pub players_topic_id: i32,
    pub poll_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            server_address: env::var("MC_SERVER_ADDRESS")?,
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")?,
            telegram_chat_id: env::var("TELEGRAM_CHAT_ID")?,
            log_topic_id: env::var("LOG_TOPIC_ID")?
                .parse()
                .map_err(|_| env::VarError::NotPresent)?,
            players_topic_id: env::var("PLAYERS_TOPIC_ID")?
                .parse()
                .map_err(|_| env::VarError::NotPresent)?,
            poll_interval_secs: env::var("POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
        })
    }
}
