use rust_mc_status::{McClient, ServerEdition};
use crate::error::{AgregationError, FetchError};

struct ServerActivityData {
    online: bool,
    players: Vec<String>,
    ping: f64
}

async fn get_raw() -> Result<ServerActivityData, FetchError> {
    let client = McClient::new()
        .with_timeout(std::time::Duration::from_secs(2))
        .with_max_parallel(1);

    // Ping a Java server (automatically uses SRV lookup if port not specified)
    let status = client.ping("qwew.space", ServerEdition::Java).await?;
    println!("Server is online: {}", status.online);
    if let Some((online, max)) = status.players() {
        println!("Players: {}/{}", online, max);
    };
    println!("ping {}", status.latency);

    Ok(())
}

