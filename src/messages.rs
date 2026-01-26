use rand::Rng;

const JOIN_MESSAGES: &[&str] = &[
    "> {player} has entered the game!",
    "> {player} just landed on the server!",
    "> {player} spawned into the world!",
    "> {player} joined the adventure!",
    "> {player} has connected to the server!",
    "> {player} is now online!",
    "> Welcome {player} to the server!",
    "> {player} has joined the party!",
    "> {player} entered the realm!",
    "> {player} stepped into the world!",
    "> {player} opened the door and walked in!",
    "> {player} appeared in a flash of light!",
    "> {player} made their grand entrance!",
    "> {player} rocked into the server!",
    "> {player} sprinted onto the server!",
];

const LEAVE_MESSAGES: &[&str] = &[
    "> {player} has left the game!",
    "> {player} vanished into thin air!",
    "> {player} sailed away!",
    "> {player} took their final bow!",
    "> {player} rode off into the sunset!",
    "> {player} logged off!",
    "> Bye, {player}!",
];

pub fn get_random_join_message(player_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let template = JOIN_MESSAGES[rng.gen_range(0..JOIN_MESSAGES.len())];
    template.replace("{player}", player_name)
}

pub fn get_random_leave_message(player_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let template = LEAVE_MESSAGES[rng.gen_range(0..LEAVE_MESSAGES.len())];
    template.replace("{player}", player_name)
}