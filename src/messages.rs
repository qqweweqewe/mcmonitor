use rand::Rng;
use std::collections::HashMap;
use std::fs;

fn load_messages(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

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
        messages[rng.gen_range(0..messages.len())].clone()
    } else {
        let regular_messages = load_messages("join_messages.json");
        regular_messages[rng.gen_range(0..regular_messages.len())].clone()
    };
    template.replace("{player}", &escape_markdown(player_name))
}

pub fn get_random_leave_message(player_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let special_messages = load_special_messages("special_leave_messages.json");
    let template = if let Some(messages) = special_messages.get(player_name) {
        messages[rng.gen_range(0..messages.len())].clone()
    } else {
        let regular_messages = load_messages("leave_messages.json");
        regular_messages[rng.gen_range(0..regular_messages.len())].clone()
    };
    template.replace("{player}", &escape_markdown(player_name))
}