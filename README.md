# Git Friends

Git Friends is a collection of tools for sharing git commit information across different platforms using MQTT as a message broker. It consists of three main components:

- **gf-hook**: A git commit hook that sends commit information to a central server
- **gf-server**: An HTTP server that receives commit webhooks and publishes them to MQTT topics
- **gf-irc**: An IRC client that listens to MQTT topics and publishes commit information to IRC channels

## Architecture

```
[Git Repository] → [gf-hook] → [gf-server] → [MQTT Broker] → [gf-irc] → [IRC Channel]
```

## Installation

### Prerequisites

- Rust 1.70 or later
- MQTT broker (e.g., Mosquitto)
- Git repositories where you want to install hooks

### Building from Source

```bash
git clone https://github.com/your-org/git-friends.git
cd git-friends
cargo build --release
```

This will create three binaries in `target/release/`:
- `gf-hook`
- `gf-server`
- `gf-irc`

## Configuration

Git Friends uses a TOML configuration file. By default, it looks for `git-friends.toml` in the current directory, but you can specify a different location:

- Using the `GIT_FRIENDS_CONFIG` environment variable
- Using the `--config` command line option
- Default locations: `./git-friends.toml`, `./config/git-friends.toml`, `/etc/git-friends.toml`

See `git-friends.toml.example` for a complete configuration example.

### Configuration Sections

#### Server Configuration
```toml
[server]
host = "localhost"
port = 8080
bind_address = "0.0.0.0:8080"
```

#### MQTT Configuration
```toml
[mqtt]
broker_host = "localhost"
broker_port = 1883
client_id = "git-friends"
username = "mqtt_user"     # Optional
password = "mqtt_pass"     # Optional
topic_prefix = "git-friends"
```

#### IRC Configuration
```toml
[irc]
server = "irc.libera.chat"
port = 6667
nick = "git-friends"
username = "git-friends"
real_name = "Git Friends Bot"
channels = ["#git-friends"]
use_tls = false
topic_filters = ["git-friends/+/+"]
```

#### Authentication Configuration
```toml
[auth]
# List of valid authentication tokens with usernames
[[auth.tokens]]
token = "your-secret-token-here"
username = "alice"

[[auth.tokens]]
token = "another-token-here"
username = "bob"

require_auth = true
```

## Usage

### 1. Start the MQTT Broker

First, ensure you have an MQTT broker running. For example, with Mosquitto:

```bash
mosquitto -v
```

### 2. Generate Authentication Token

Generate a secure authentication token for a user:

```bash
./gf-server --generate-token alice
```

Add this token to your configuration file.

### 3. Start the Server

```bash
./gf-server --config git-friends.toml
```

The server will:
- Listen for HTTP POST requests on the configured port
- Validate authentication tokens
- Publish commit information to MQTT topics

### 4. Start the IRC Client

```bash
./gf-irc --config git-friends.toml
```

The IRC client will:
- Connect to the configured IRC server and channels
- Subscribe to MQTT topics
- Forward commit messages to IRC channels

### 5. Install Git Hooks

In your git repository, install the commit hook:

```bash
# In your git repository
./gf-hook --dry-run  # Test the hook first
```

To install as a git hook, create a post-commit hook:

```bash
#!/bin/bash
# .git/hooks/post-commit
/path/to/gf-hook --server-url http://your-server:8080 --token your-token-here
```

Make it executable:

```bash
chmod +x .git/hooks/post-commit
```

## Command Line Options

### gf-hook

```bash
gf-hook [OPTIONS]

OPTIONS:
    -s, --server-url <URL>       The URL of the gf-server [default: http://localhost:8080]
    -t, --token <TOKEN>          Authentication token
    -c, --commit <COMMIT_HASH>   Specific commit hash to process
        --github-actions         Force GitHub Actions mode (auto-detected by default)
    -d, --dry-run                Don't actually send the request
    -h, --help                   Print help information
```

### gf-server

```bash
gf-server [OPTIONS]

OPTIONS:
    -c, --config <FILE>          Configuration file path
    -b, --bind <ADDRESS>         Bind address (e.g., 0.0.0.0:8080)
        --generate-token <USER>  Generate a new authentication token for username
    -h, --help                   Print help information
```

### gf-irc

```bash
gf-irc [OPTIONS]

OPTIONS:
    -c, --config <FILE>          Configuration file path
    -n, --nick <NICKNAME>        IRC nickname
    -s, --server <SERVER>        IRC server
        --channels <CHANNELS>    Comma-separated list of IRC channels
        --mqtt-topics <TOPICS>   Comma-separated list of MQTT topics to subscribe to
    -h, --help                   Print help information
```

## MQTT Topics

Git Friends uses hierarchical MQTT topics with optional username prefixes:

**With usernames:**
```
{topic_prefix}/{username}/{repository_identifier}/{committer_name}
```

**Without usernames:**
```
{topic_prefix}/{repository_identifier}/{committer_name}
```

Examples:
- `git-friends/alice/github_com_user_repo/john_doe`
- `git-friends/github_com_user_repo/john_doe`

Repository URLs are automatically sanitized to create valid topic names.

## IRC Message Format

Commit messages are formatted for IRC as:

```
[abc1234] repo-name by Author Name (branch): Commit message - file1.rs, file2.rs
```

## GitHub Actions Integration

Git Friends can be easily integrated with GitHub Actions. See [GITHUB_ACTIONS.md](GITHUB_ACTIONS.md) for detailed instructions.

### Quick Setup

1. Add secrets to your repository:
   - `GIT_FRIENDS_SERVER_URL`: Your server URL
   - `GIT_FRIENDS_TOKEN`: Authentication token

2. Create workflow file:
```yaml
name: Git Friends Notification
on:
  push:
    branches: [ main ]
jobs:
  notify:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    - uses: your-org/git-friends@main
      with:
        server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
        token: ${{ secrets.GIT_FRIENDS_TOKEN }}
```

## Environment Variables

- `GIT_FRIENDS_CONFIG`: Path to configuration file
- `GIT_FRIENDS_TOKEN`: Authentication token (for gf-hook)
- `GIT_COMMIT`: Commit hash (automatically set by git hooks)
- `GITHUB_SHA`: GitHub Actions commit hash
- `RUST_LOG`: Logging level (e.g., `info`, `debug`)

## Security

- All HTTP requests to gf-server require authentication via Bearer tokens
- Tokens are generated using UUIDs and base64 encoding
- MQTT and IRC connections can be secured with TLS/SSL
- No sensitive information is logged

## Development

### Running Tests

```bash
cargo test
```

### Logging

Set the `RUST_LOG` environment variable to control logging:

```bash
RUST_LOG=info ./gf-server
RUST_LOG=debug ./gf-irc
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the GPL License - see the LICENSE file for details.

## Troubleshooting

### Common Issues

1. **Connection refused**: Ensure the MQTT broker is running and accessible
2. **Authentication failed**: Check that tokens match between client and server
3. **IRC connection issues**: Verify IRC server details and network connectivity
4. **Git hook not working**: Check file permissions and paths

### Debug Mode

Run with debug logging to see detailed information:

```bash
RUST_LOG=debug ./gf-server
RUST_LOG=debug ./gf-irc
```

## Examples

### Basic Setup

1. Start Mosquitto MQTT broker
2. Generate token: `./gf-server --generate-token`
3. Update configuration with the token
4. Start server: `./gf-server`
5. Start IRC client: `./gf-irc`
6. Install git hook in your repository

### Multiple Repositories

Configure repository mappings in your config file:

```toml
[git.repository_mappings]
"https://github.com/user/repo1" = "repo1"
"https://github.com/user/repo2" = "repo2"
```

### Custom IRC Channels

```bash
./gf-irc --channels "#dev,#commits,#notifications"
```

### Filtering MQTT Topics

```bash
./gf-irc --mqtt-topics "git-friends/repo1/+,git-friends/repo2/alice"
```
