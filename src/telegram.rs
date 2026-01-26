use crate::error::TelegramError;

pub async fn send_message(
    bot_token: &str,
    chat_id: &str,
    message: &str,
) -> Result<(), TelegramError> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": message,
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(TelegramError::ApiError(response.status().to_string()));
    }

    Ok(())
}

pub async fn handle_all_command(
    bot_token: &str,
    chat_id: &str,
    server_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match crate::fetch::get_server_status(server_address).await {
        Ok(status) => {
            let message = match status.players {
                Some(players) if !players.is_empty() => {
                    format!("Players online ({}):\n> {}", players.len(), players.join("\n> "))
                },
                Some(_) => "> No players online".to_string(),
                None => "> Pretty empty here".to_string(),
            };
            
            send_message(bot_token, chat_id, &message).await?;
        },
        Err(_) => {
            send_message(bot_token, chat_id, "> Server is offline or unreachable").await?;
        }
    }
    
    Ok(())
}

pub async fn check_commands(
    bot_token: &str,
    chat_id: &str,
    server_address: &str,
    last_update_id: &mut i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.telegram.org/bot{}/getUpdates?offset={}", 
        bot_token, 
        *last_update_id + 1
    );
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let updates: serde_json::Value = response.json().await?;
    
    if let Some(result) = updates["result"].as_array() {
        for update in result {
            if let Some(update_id) = update["update_id"].as_i64() {
                *last_update_id = update_id;
                
                if let Some(message) = update["message"].as_object() {
                    if let Some(text) = message["text"].as_str() {
                        if text == "/all" {
                            handle_all_command(bot_token, chat_id, server_address).await?;
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}
