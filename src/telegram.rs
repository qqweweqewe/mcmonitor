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
