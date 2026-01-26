use rust_mc_status::{McClient, ServerEdition};
use crate::error::FetchError;

pub struct ServerActivityData {
    pub players: Option<Vec<String>>,
    pub ping: f64,
}

pub async fn get_server_status(server_address: &str) -> Result<ServerActivityData, FetchError> {
    let client = McClient::new()
        .with_timeout(std::time::Duration::from_secs(2))
        .with_max_parallel(1);


    let status = client.ping(server_address, ServerEdition::Java).await?;

    if !status.online {
        return Err("server offline".into())
    }

    let mut players: Option<Vec<String>> = Option::None;

    if let rust_mc_status::ServerData::Java(java) = status.data {
        let java_players = java.players;
        players = match java_players.sample {
            Some(player_vec) => {
                Some(player_vec.into_iter().map(|p| p.name).collect())
            },
            None => None
        };
    }    

    Ok(ServerActivityData {
        players,
        ping: status.latency,
    })
}

