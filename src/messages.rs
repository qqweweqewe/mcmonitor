use rand::Rng;

const JOIN_MESSAGES: &[&str] = &[
    "🎮 {player} has entered the game!",
    "🚀 {player} just landed on the server!",
    "⚡ {player} spawned into the world!",
    "🎯 {player} joined the adventure!",
    "🌟 {player} has connected to the server!",
    "🔥 {player} is now online!",
    "🎊 Welcome {player} to the server!",
    "🎈 {player} has joined the party!",
    "⭐ {player} entered the realm!",
    "🎪 {player} stepped into the world!",
    "🚪 {player} opened the door and walked in!",
    "🌈 {player} appeared in a flash of light!",
    "🎭 {player} made their grand entrance!",
    "🎸 {player} rocked into the server!",
    "🏃 {player} sprinted onto the server!",
];

pub fn get_random_join_message(player_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let template = JOIN_MESSAGES[rng.gen_range(0..JOIN_MESSAGES.len())];
    template.replace("{player}", player_name)
}