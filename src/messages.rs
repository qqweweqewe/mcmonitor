use rand::Rng;
use std::collections::HashMap;
use std::fs;

const JOIN_MESSAGES: &[&str] = &[
    r"⟐ *{player}* бля опять ты",
    r"⟐ *{player}* на месте🫡",
    r"⟐ привет *{player}*\!",
    r"⟐ *{player}* залетает на серв",
    r"⟐ лютый *{player}*",
    r"⟐ *{player}* снова тут\!",
    r"⟐ *{player}* просто взял вошел",
    r"⟐ кто составит компанию *{player}*?",
    r"⟐ *{player}* материализовался",
    r"⟐ *{player}* присоединяется",
    r"⟐ *{player}* пожаловал",
];

const LEAVE_MESSAGES: &[&str] = &[
    r"⁛ *{player}* ушел в закат",
    r"⁛ *{player}* мама позвала кушать",
    r"⁛ *{player}* наконец съебал",
    r"⁛ *{player}* до связи\!",
    r"⁛ *{player}* ну куда собрался нууу вернись",
    r"⁛ Adios, *{player}*\!",
];

// IMPORTANT PART
fn load_special_messages(file_path: &str) -> HashMap<String, Vec<String>> {
    fs::read_to_string(file_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

fn escape_markdown(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|' | '{' | '}' | '.' | '!' => format!("\\{}", c),
            _ => c.to_string(),
        })
        .collect()
}

pub fn get_random_join_message(player_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let special_messages = load_special_messages("special_join_messages.json");
    let template = if let Some(messages) = special_messages.get(player_name) {
        &messages[rng.gen_range(0..messages.len())]
    } else {
        JOIN_MESSAGES[rng.gen_range(0..JOIN_MESSAGES.len())]
    };
    template.replace("{player}", &escape_markdown(player_name))
}

pub fn get_random_leave_message(player_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let special_messages = load_special_messages("special_leave_messages.json");
    let template = if let Some(messages) = special_messages.get(player_name) {
        &messages[rng.gen_range(0..messages.len())]
    } else {
        LEAVE_MESSAGES[rng.gen_range(0..LEAVE_MESSAGES.len())]
    };
    template.replace("{player}", &escape_markdown(player_name))
}