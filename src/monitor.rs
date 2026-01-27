use std::collections::HashSet;
use std::time::Duration;
use tokio::time;
use crate::{config::Config, fetch, telegram, messages};

pub struct Monitor {
    config: Config,
    previous_players: HashSet<String>,
    players_message_id: Option<i32>,
    last_players_message: Option<String>,
}

impl Monitor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            previous_players: HashSet::new(),
            players_message_id: None,
            last_players_message: None,
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

        let current_players: HashSet<String> = status.players
            .unwrap_or_default()
            .into_iter()
            .collect();
        
        let new_players: Vec<_> = current_players
            .difference(&self.previous_players)
            .cloned()
            .collect();
            
        let left_players: Vec<_> = self.previous_players
            .difference(&current_players)
            .cloned()
            .collect();

        // Send join/leave messages to log topic
        for player in &new_players {
            let message = messages::get_random_join_message(player);
            let _ = telegram::send_message_to_topic(
                &self.config.telegram_bot_token,
                &self.config.telegram_chat_id,
                self.config.log_topic_id,
                &message,
            )
            .await;
        }
        
        for player in &left_players {
            let message = messages::get_random_leave_message(player);
            let _ = telegram::send_message_to_topic(
                &self.config.telegram_bot_token,
                &self.config.telegram_chat_id,
                self.config.log_topic_id,
                &message,
            )
            .await;
        }

        if let Err(e) = self.update_players_message(&current_players).await {
            eprintln!("Failed to update players message: {}", e);
        }

        self.previous_players = current_players;
        Ok(())
    }

    async fn update_players_message(&mut self, players: &HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
        let message = if players.is_empty() {
            format!(
                "```\n┌─────────────────────┐\n│   SERVER IS EMPTY   │\n└─────────────────────┘\n```"
            )
        } else {
            let player_list: Vec<String> = players.iter()
                .map(|p| format!("│ ◦ {}\n", p))
                .collect();
            format!(
                "```\n┌─────────────────────┐\n│ PLAYERS ONLINE ({:>2}) │\n├─────────────────────┤\n{}└─────────────────────┘\n```",
                players.len(),
                player_list.join("")
            )
        };

        if self.last_players_message.as_ref() == Some(&message) {
            return Ok(());
        }

        match self.players_message_id {
            Some(message_id) => {
                telegram::edit_message(
                    &self.config.telegram_bot_token,
                    &self.config.telegram_chat_id,
                    message_id,
                    &message,
                ).await?;
            },
            None => {
                let message_id = telegram::send_message_to_topic(
                    &self.config.telegram_bot_token,
                    &self.config.telegram_chat_id,
                    self.config.players_topic_id,
                    &message,
                ).await?;
                self.players_message_id = Some(message_id);
            }
        }

        self.last_players_message = Some(message);
        Ok(())
    }
}
