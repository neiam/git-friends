# GitHub Actions Integration

Git Friends can be easily integrated with GitHub Actions to automatically send commit notifications to your IRC channels via MQTT.

## Quick Start

1. **Add secrets to your repository:**
   - `GIT_FRIENDS_SERVER_URL`: Your gf-server URL (e.g., `https://git-friends.example.com`)
   - `GIT_FRIENDS_TOKEN`: Authentication token from your server

2. **Create a workflow file** (`.github/workflows/git-friends.yml`):

```yaml
name: Git Friends Notification

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  notify:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        fetch-depth: 0  # Get full history for better git info
    
    - name: Send Git Friends notification
      uses: your-org/git-friends@main
      with:
        server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
        token: ${{ secrets.GIT_FRIENDS_TOKEN }}
```

## Advanced Configuration

### Custom gf-hook Version

```yaml
- name: Send Git Friends notification
  uses: your-org/git-friends@main
  with:
    server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
    token: ${{ secrets.GIT_FRIENDS_TOKEN }}
    gf-hook-version: v1.0.0  # Use specific version
```

### Dry Run Mode

```yaml
- name: Send Git Friends notification (dry run)
  uses: your-org/git-friends@main
  with:
    server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
    token: ${{ secrets.GIT_FRIENDS_TOKEN }}
    dry-run: true
```

### Manual Binary Usage

If you prefer to use the binary directly:

```yaml
- name: Download and run gf-hook
  run: |
    # Download the binary
    curl -L -o gf-hook https://github.com/your-org/git-friends/releases/latest/download/gf-hook
    chmod +x gf-hook
    
    # Run it
    ./gf-hook --server-url ${{ secrets.GIT_FRIENDS_SERVER_URL }} --github-actions
  env:
    GIT_FRIENDS_TOKEN: ${{ secrets.GIT_FRIENDS_TOKEN }}
```

## Available Environment Variables

The following GitHub Actions environment variables are automatically used:

- `GITHUB_SHA`: Commit hash
- `GITHUB_REPOSITORY`: Repository name (e.g., `owner/repo`)
- `GITHUB_SERVER_URL`: GitHub server URL (usually `https://github.com`)
- `GITHUB_REF_NAME`: Branch name
- `GITHUB_ACTOR`: User who triggered the action
- `GITHUB_EVENT_PATH`: Path to event payload JSON
- `GITHUB_ACTIONS`: Indicates we're in GitHub Actions

## Event Types Supported

- **Push events**: Commit information from the pushed commit
- **Pull request events**: Pull request title and information
- **Manual dispatch**: Uses the current HEAD commit

## MQTT Topic Structure

With usernames, the MQTT topics follow this pattern:
```
git-friends/{username}/{repository}/{committer}
```

Example:
```
git-friends/alice/github_com_owner_repo/john_doe
```

## IRC Message Format

Messages appear in IRC like this:
```
[abc1234] owner/repo by John Doe (main): Fix authentication bug - src/auth.rs, tests/auth_test.rs
```

## Troubleshooting

### Common Issues

1. **No commit information**: Ensure `fetch-depth: 0` in checkout action
2. **Authentication failed**: Check that your token is valid and has the correct username
3. **Network issues**: Verify your server URL is accessible from GitHub Actions

### Debug Mode

Enable debug logging:

```yaml
- name: Send Git Friends notification
  uses: your-org/git-friends@main
  with:
    server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
    token: ${{ secrets.GIT_FRIENDS_TOKEN }}
  env:
    RUST_LOG: debug
```

### Testing

Use dry-run mode to test without sending actual notifications:

```yaml
- name: Test Git Friends notification
  uses: your-org/git-friends@main
  with:
    server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
    token: ${{ secrets.GIT_FRIENDS_TOKEN }}
    dry-run: true
```

## Action Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `server-url` | Git Friends server URL | Yes | - |
| `token` | Authentication token | Yes | - |
| `gf-hook-version` | Version of gf-hook to use | No | `latest` |
| `dry-run` | Run in dry-run mode | No | `false` |

## Example Complete Workflow

```yaml
name: Git Friends Notifications

on:
  push:
    branches: [ main, develop, feature/* ]
  pull_request:
    branches: [ main ]
  release:
    types: [ published ]

jobs:
  notify-commits:
    if: github.event_name == 'push'
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    
    - name: Notify commit
      uses: your-org/git-friends@main
      with:
        server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
        token: ${{ secrets.GIT_FRIENDS_TOKEN }}
  
  notify-prs:
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    
    - name: Notify PR
      uses: your-org/git-friends@main
      with:
        server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
        token: ${{ secrets.GIT_FRIENDS_TOKEN }}
  
  notify-releases:
    if: github.event_name == 'release'
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    
    - name: Notify release
      uses: your-org/git-friends@main
      with:
        server-url: ${{ secrets.GIT_FRIENDS_SERVER_URL }}
        token: ${{ secrets.GIT_FRIENDS_TOKEN }}
```
