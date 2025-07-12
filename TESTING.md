# Git Friends Test Client (gf-tester)

The `gf-tester` binary is a test client that sends mock commit messages to test the Git Friends system. It's useful for testing MQTT connectivity, IRC integration, and overall system functionality without needing actual git repositories.

## Features

- Sends realistic mock commit messages
- Configurable number of commits and intervals
- Supports continuous mode for long-running tests
- Uses unique client ID (`{base_client_id}/tester`) to avoid conflicts
- Includes varied commit messages and file changes for realistic testing

## Usage

### Basic Usage

Send 5 test commits with 2-second intervals:
```bash
./gf-tester
```

### Advanced Usage

```bash
# Send 10 commits with 1-second intervals
./gf-tester --count 10 --interval 1

# Use custom configuration file
./gf-tester --config /path/to/config.toml

# Send commits continuously (until Ctrl+C)
./gf-tester --continuous --interval 5

# Test with custom repository and author
./gf-tester --repo "https://github.com/myorg/myrepo" --author "John Doe"

# Test with authentication
./gf-tester --username testuser --count 3
```

## Command Line Options

- `-c, --config <FILE>` - Configuration file path
- `-n, --count <NUMBER>` - Number of test commits to send (default: 5)
- `-i, --interval <SECONDS>` - Interval between commits in seconds (default: 2)
- `-u, --username <USERNAME>` - Username for authentication (if required)
- `-r, --repo <REPO_URL>` - Mock repository URL (default: https://github.com/test/mock-repo)
- `-a, --author <AUTHOR>` - Mock author name (default: "Test Author")
- `-b, --branch <BRANCH>` - Mock branch name (default: main)
- `--continuous` - Send commits continuously until interrupted

## Mock Data

The tester generates realistic commit data including:

- **Commit Messages**: Cycles through 20 different realistic commit messages
- **File Changes**: Rotates through 10 different sets of files
- **Unique Hashes**: Generates unique commit hashes using UUIDs
- **Timestamps**: Uses current timestamp for each commit
- **Email Addresses**: Generates email addresses based on author names

## Client ID

The tester automatically appends "/tester" to the configured MQTT client ID to avoid conflicts with other components. For example, if your config has `client_id = "git-friends"`, the tester will use `git-friends/tester`.

## Testing Scenarios

### 1. Basic Functionality Test
```bash
./gf-tester --count 3 --interval 1
```

### 2. Load Testing
```bash
./gf-tester --continuous --interval 0.5
```

### 3. Authentication Testing
```bash
./gf-tester --username testuser --count 5
```

### 4. Multi-Repository Testing
```bash
./gf-tester --repo "https://github.com/org1/repo1" --author "Alice" --count 2
./gf-tester --repo "https://github.com/org2/repo2" --author "Bob" --count 2
```

## Integration with Other Components

The tester works seamlessly with:
- **gf-irc**: IRC client will receive and display the mock commits
- **gf-server**: Server can process the commits if running
- **MQTT Broker**: All components can see the test messages

## Component Client IDs

Each binary now uses a unique client ID to avoid MQTT conflicts:
- `gf-tester`: `{base_client_id}/tester`
- `gf-irc`: `{base_client_id}/irc`
- `gf-server`: `{base_client_id}/server`
- `gf-hook`: Uses base client ID (unchanged)
