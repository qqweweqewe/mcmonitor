# MCMonitor

A Rust-based Minecraft server monitoring bot that tracks player activity and sends notifications to Telegram.

## Features

- **Real-time Player Monitoring**: Tracks players joining and leaving the server
- **Live Player Board**: Displays currently online players with session playtime
- **Custom Messages**: Randomized join/leave messages with special player support
- **Persistent State**: Maintains player data across bot restarts
- **Telegram Integration**: Posts to specific topics in Telegram groups
- **Clean ASCII Interface**: Terminal-style player display without emojis

## Requirements

- Rust 1.70+
- Minecraft server with query enabled
- Telegram bot token
- Telegram group with topics enabled

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd mcmonitor
```

2. Copy the environment template:
```bash
cp env_example .env
```

3. Configure your environment variables in `.env`:
```env
MC_SERVER_ADDRESS=your.minecraft.server:25565
TELEGRAM_BOT_TOKEN=your_bot_token_here
TELEGRAM_CHAT_ID=-1001234567890
POLL_INTERVAL_SECS=10
LOG_TOPIC_ID=123
PLAYERS_TOPIC_ID=456
```

4. Build and run:
```bash
cargo build --release
cargo run
```

## Configuration

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `MC_SERVER_ADDRESS` | Minecraft server address with port | `mc.example.com:25565` |
| `TELEGRAM_BOT_TOKEN` | Bot token from @BotFather | `1234567890:ABC...` |
| `TELEGRAM_CHAT_ID` | Telegram group chat ID | `-1001234567890` |
| `POLL_INTERVAL_SECS` | Server polling interval in seconds | `10` |
| `LOG_TOPIC_ID` | Topic ID for join/leave messages | `123` |
| `PLAYERS_TOPIC_ID` | Topic ID for player board | `456` |

### Message Customization

The bot uses JSON files for message templates:

- `join_messages.json` - Default join messages
- `leave_messages.json` - Default leave messages
- `special_join_messages.json` - Player-specific join messages (optional)
- `special_leave_messages.json` - Player-specific leave messages (optional)

Message templates support `{player}` placeholder for player names.

Example `join_messages.json`:
```json
[
  "⟐ *{player}* joined the server!",
  "⟐ Welcome back *{player}*!",
  "⟐ *{player}* has entered the game"
]
```

### Special Player Messages

Create `special_join_messages.json` and `special_leave_messages.json` for player-specific messages:

```json
{
  "PlayerName": [
    "⟐ The legend *{player}* has arrived!",
    "⟐ *{player}* graces us with their presence"
  ]
}
```

## Telegram Setup

### 1. Create a Bot

1. Message @BotFather on Telegram
2. Use `/newbot` command
3. Follow instructions to get your bot token

### 2. Get Chat ID

1. Add your bot to the target group
2. Send a message in the group
3. Visit: `https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates`
4. Find your chat ID in the response

### 3. Enable Topics

1. Go to group settings
2. Enable "Topics" feature
3. Create topics for logs and player board
4. Get topic IDs from message URLs or API

### 4. Bot Permissions

Make your bot an admin with these permissions:
- Post Messages
- Manage Topics (required for closed topics so only bot can write to them)

## Player Board Display

The bot maintains a live player board showing:

```
╭─────────────────────────────────╮
│        PLAYERS ONLINE  2        │
├─────────────────────────────────┤
│ • Alice           01:23:40      │
│ • Bob             00:12:30      │
╰─────────────────────────────────╯
```

Features:
- Real-time player count
- Session playtime in HH:MM:SS format
- Alphabetically sorted player list
- Updates only when changes occur
- Persists across bot restarts

## State Persistence

The bot saves state to `monitor_state.json` including:
- Previously online players
- Player join timestamps
- Telegram message IDs

This prevents duplicate notifications on restart and maintains accurate playtime tracking.

## Dependencies

- `rust-mc-status` - Minecraft server status queries
- `tokio` - Async runtime
- `reqwest` - HTTP client for Telegram API
- `serde` - JSON serialization
- `dotenv` - Environment variable loading
- `rand` - Random message selection

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running

```bash
# Development
cargo run

# Production
./target/release/mcmonitor
```

## Troubleshooting

### Common Issues

1. **Bot can't post to topics**: Ensure bot has "Manage Topics" permission
2. **Server connection failed**: Check server address and query port
3. **No messages sent**: Verify bot token and chat ID
4. **State file errors**: Check file permissions in working directory

### Logs

The application outputs status information to stdout:
- Configuration loading
- Server connection status
- Error messages for debugging

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Support

For issues and questions, please open an issue on the repository.