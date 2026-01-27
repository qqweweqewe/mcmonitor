use rand::Rng;

const JOIN_MESSAGES: &[&str] = &[
    r"▶ *{player}* has entered the game\!",
    r"▶ *{player}* just landed on the server\!",
    r"▶ *{player}* spawned into the world\!",
    r"▶ *{player}* joined the adventure\!",
    r"▶ *{player}* has connected to the server\!",
    r"▶ *{player}* is now online\!",
    r"▶ Welcome *{player}* to the server\!",
    r"▶ *{player}* has joined the party\!",
    r"▶ *{player}* entered the realm\!",
    r"▶ *{player}* stepped into the world\!",
    r"▶ *{player}* opened the door and walked in\!",
    r"▶ *{player}* appeared in a flash of light\!",
    r"▶ *{player}* made their grand entrance\!",
    r"▶ *{player}* rocked into the server\!",
    r"▶ *{player}* sprinted onto the server\!",
];

const LEAVE_MESSAGES: &[&str] = &[
    r"◀ *{player}* has left the game\!",
    r"◀ *{player}* vanished into thin air\!",
    r"◀ *{player}* sailed away\!",
    r"◀ *{player}* took their final bow\!",
    r"◀ *{player}* rode off into the sunset\!",
    r"◀ *{player}* logged off\!",
    r"◀ Bye, *{player}*\!",
];

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
    let template = JOIN_MESSAGES[rng.gen_range(0..JOIN_MESSAGES.len())];
    template.replace("{player}", &escape_markdown(player_name))
}

pub fn get_random_leave_message(player_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let template = LEAVE_MESSAGES[rng.gen_range(0..LEAVE_MESSAGES.len())];
    template.replace("{player}", &escape_markdown(player_name))
}