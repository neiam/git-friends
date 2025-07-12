# Git Friends Environment-Based Hook Setup

This directory contains an environment-based setup for Git Friends post-commit hooks, which is more flexible and easier to manage than hardcoding paths and tokens in your git hooks.

## Files

- **`example-post-commit-environ`** - Environment-based post-commit hook script
- **`.git-friends.profile`** - Profile file with environment variable definitions
- **`install-hook-environ.sh`** - Installation script to set everything up

## Quick Setup

1. **Run the installer:**
   ```bash
   ./install-hook-environ.sh
   ```

2. **Source the profile:**
   ```bash
   source ~/.git-friends.profile
   ```

3. **Test the configuration:**
   ```bash
   test_git_friends
   ```

4. **Make a commit to test:**
   ```bash
   git add .
   git commit -m "Test commit"
   ```

## Manual Setup

### 1. Set up the profile

Copy the profile to your home directory:
```bash
cp .git-friends.profile ~/.git-friends.profile
```

Edit `~/.git-friends.profile` and set the correct path to your `gf-hook` binary:
```bash
export GF_HOOK_PATH="/path/to/your/gf-hook"
```

### 2. Source the profile

Add this to your `~/.bashrc` or `~/.zshrc`:
```bash
if [ -f "$HOME/.git-friends.profile" ]; then
    source "$HOME/.git-friends.profile"
fi
```

Then reload your shell or run:
```bash
source ~/.git-friends.profile
```

### 3. Install the hook

For the current repository:
```bash
cp example-post-commit-environ .git/hooks/post-commit
chmod +x .git/hooks/post-commit
```

For other repositories:
```bash
cp example-post-commit-environ /path/to/other/repo/.git/hooks/post-commit
chmod +x /path/to/other/repo/.git/hooks/post-commit
```

## Configuration

The environment variables used by the hook are:

### Required
- **`GF_HOOK_PATH`** - Path to the `gf-hook` binary

### Optional
- **`GF_SERVER_URL`** - URL of the gf-server (default: `http://localhost:8080`)
- **`GF_TOKEN`** - Authentication token (only if server requires auth)
- **`GF_CONFIG`** - Path to git-friends config file

## Helper Functions

The profile includes several helper functions:

### `test_git_friends`
Tests your Git Friends configuration and runs a dry-run:
```bash
test_git_friends
```

### `show_git_friends_config`
Shows all current Git Friends environment variables:
```bash
show_git_friends_config
```

### `install_git_friends_hook`
Installs the hook in the current repository:
```bash
install_git_friends_hook
```

## Advanced Usage

### Multiple Servers
You can override the server URL per repository by setting `GF_SERVER_URL` before committing:
```bash
GF_SERVER_URL="https://other-server.com" git commit -m "message"
```

### Debugging
Enable detailed logging:
```bash
export RUST_LOG="debug"
git commit -m "test commit"
```

### Different Tokens per Repository
You can set different tokens for different repositories by creating a local `.git-friends.profile` in each repository:
```bash
# In specific repository
cat > .git-friends.profile << EOF
export GF_TOKEN="repository-specific-token"
EOF
```

The hook will automatically source this if it exists.

## Troubleshooting

### Hook not running
1. Check if the hook is executable:
   ```bash
   ls -la .git/hooks/post-commit
   ```

2. Test manually:
   ```bash
   .git/hooks/post-commit
   ```

### Configuration issues
1. Check your configuration:
   ```bash
   show_git_friends_config
   ```

2. Test the configuration:
   ```bash
   test_git_friends
   ```

### Authentication errors
1. Make sure your server is running and accessible
2. Check if authentication is required on the server
3. Verify your token is correct
4. Test with a dry run:
   ```bash
   gf-hook --dry-run
   ```

## Comparison with Simple Hook

| Feature | Simple Hook | Environment Hook |
|---------|-------------|------------------|
| Setup complexity | Low | Medium |
| Flexibility | Low | High |
| Multiple repos | Manual copy | Automatic |
| Token management | Hardcoded | Environment variable |
| Server switching | Edit each hook | Change environment |
| Debugging | Limited | Full logging support |
| Maintenance | High | Low |

## Migration from Simple Hook

If you're already using the simple `example-post-commit` hook:

1. Run the installer:
   ```bash
   ./install-hook-environ.sh
   ```

2. Your old hook will be automatically backed up

3. Source the new profile:
   ```bash
   source ~/.git-friends.profile
   ```

4. Test the new setup:
   ```bash
   test_git_friends
   ```
