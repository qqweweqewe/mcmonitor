use std::collections::HashSet;
use std::time::Duration;
use tokio::time;
use crate::{config::Config, fetch, telegram, messages};

pub struct Monitor {
    config: Config,
    previous_players: HashSet<String>,
}

impl Monitor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            previous_players: HashSet::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = time::interval(Duration::from_secs(self.config.poll_interval_secs));

        loop {
            interval.tick().await;
            
            if let Err(e) = self.check_server().await {
                eprintln!("Error checking server: {}", e);
            }
        }
    }

    async fn check_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let status = fetch::get_server_status(&self.config.server_address).await?;

        let current_players: HashSet<String> = match status.players {
            Some(players) => players.into_iter().collect(),

            None => {

                self.previous_players.clear();

                return Ok(());

            }

        };
        
        let new_players: Vec<_> = current_players
            .difference(&self.previous_players)
            .cloned()
            .collect();

        for player in &new_players {
            let message = messages::get_random_join_message(player);
            telegram::send_message(
                &self.config.telegram_bot_token,
                &self.config.telegram_chat_id,
                &message,
            )
            .await?;
        }

        self.previous_players = current_players;
        Ok(())
    }
}
