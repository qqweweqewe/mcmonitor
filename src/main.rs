mod fetch;
mod error;
mod config;
mod telegram;
mod monitor;

use config::Config;
use monitor::Monitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    println!("loading config");
    let config = Config::from_env()?;

    println!("monitor init");
    let mut monitor = Monitor::new(config);
    monitor.run().await
}
