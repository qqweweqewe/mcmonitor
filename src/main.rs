mod fetch;
mod error;
mod config;
mod telegram;
mod monitor;
mod messages;

use config::Config;
use monitor::Monitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    println!("Loading config...");
    let config = Config::from_env()?;
    println!("Done. Current config:\ntg_chat_id: {}\nserver_addr: {}\npoll_interval: {}s\n", &config.telegram_chat_id, &config.server_address, &config.poll_interval_secs);

    println!("Monitor init...");
    let mut monitor = Monitor::new(config);
    
    println!("--------------------\nAll up and running.");
    monitor.run().await
}
