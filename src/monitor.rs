use std::collections::{HashSet, HashMap};
use std::time::{Duration, Instant};
use tokio::time;
use serde::{Deserialize, Serialize};
use crate::{config::Config, fetch, telegram, messages};

#[derive(Serialize, Deserialize, Default)]
struct MonitorState {
    previous_players: HashSet<String>,
    players_message_id: Option<i32>,
    last_players_message: Option<String>,
    player_join_times: HashMap<String, u64>,
}

pub struct Monitor {
    config: Config,
    previous_players: HashSet<String>,
    players_message_id: Option<i32>,
    last_players_message: Option<String>,
    player_join_times: HashMap<String, Instant>,
}

impl Monitor {
    pub fn new(config: Config) -> Self {
        let state = Self::load_state().unwrap_or_default();
        let now = Instant::now();
        let player_join_times = state.player_join_times.into_iter()
            .map(|(player, secs)| (player, now - Duration::from_secs(secs)))
            .collect();
        Self {
            config,
            previous_players: state.previous_players,
            players_message_id: state.players_message_id,
            last_players_message: state.last_players_message,
            player_join_times,
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
            self.player_join_times.insert(player.clone(), Instant::now());
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
            self.player_join_times.remove(player);
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
        let _ = self.save_state();
        Ok(())
    }

    async fn update_players_message(&mut self, players: &HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
        let message = if players.is_empty() {
            format!(
                "```\n╭─────────────────────────────────╮\n│         [ SERVER EMPTY ]        │\n╰─────────────────────────────────╯\n```"
            )
        } else {
            let mut sorted_players: Vec<_> = players.iter().collect();
            sorted_players.sort();
            let player_list: Vec<String> = sorted_players.iter()
                .map(|p| {
                    let playtime = self.player_join_times.get(*p)
                        .map(|join_time| join_time.elapsed())
                        .unwrap_or(Duration::ZERO);
                    let total_secs = playtime.as_secs();
                    let hours = total_secs / 3600;
                    let mins = (total_secs % 3600) / 60;
                    let secs = total_secs % 60;
                    format!("│ • {:<15} {:02}:{:02}:{:02}      │\n", p, hours, mins, secs)
                })
                .collect();
            format!(
                "```\n╭─────────────────────────────────╮\n│        PLAYERS ONLINE {:>2}        │\n├─────────────────────────────────┤\n{}╰─────────────────────────────────╯\n```",
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

    fn load_state() -> Result<MonitorState, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string("monitor_state.json")?;
        Ok(serde_json::from_str(&data)?)
    }

    fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let now = Instant::now();
        let player_join_times = self.player_join_times.iter()
            .map(|(player, join_time)| (player.clone(), now.duration_since(*join_time).as_secs()))
            .collect();
        let state = MonitorState {
            previous_players: self.previous_players.clone(),
            players_message_id: self.players_message_id,
            last_players_message: self.last_players_message.clone(),
            player_join_times,
        };
        let data = serde_json::to_string_pretty(&state)?;
        std::fs::write("monitor_state.json", data)?;
        Ok(())
    }
}
