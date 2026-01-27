use crate::error::TelegramError;

pub async fn send_message_to_topic(
    bot_token: &str,
    chat_id: &str,
    topic_id: i32,
    message: &str,
) -> Result<i32, TelegramError> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "message_thread_id": topic_id,
            "text": message,
            "parse_mode": "MarkdownV2"
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        eprintln!("Telegram API error response: {}", error_text);
        return Err(TelegramError::ApiError(format!("{} - {}", status, error_text)));
    }

    let result: serde_json::Value = response.json().await?;
    let message_id = result
        .get("result")
        .and_then(|r| r.get("message_id"))
        .and_then(|id| id.as_i64())
        .unwrap_or(0) as i32;
    Ok(message_id)
}

pub async fn edit_message(
    bot_token: &str,
    chat_id: &str,
    message_id: i32,
    new_text: &str,
) -> Result<(), TelegramError> {
    let url = format!("https://api.telegram.org/bot{}/editMessageText", bot_token);
    
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "message_id": message_id,
            "text": new_text,
            "parse_mode": "MarkdownV2"
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(TelegramError::ApiError(response.status().to_string()));
    }

    Ok(())
}
