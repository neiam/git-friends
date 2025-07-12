# Git Friends Profile Configuration
# ~/.git-friends.profile
#
# This file contains environment variable settings for Git Friends
# Source this file in your shell profile or before running git operations
#
# Usage:
#   source ~/.git-friends.profile
#   
# Or add to your ~/.bashrc or ~/.zshrc:
#   source ~/.git-friends.profile

# =============================================================================
# Git Friends Configuration
# =============================================================================

# Path to the gf-hook binary (REQUIRED)
# Change this to the actual path where you installed/built gf-hook
export GF_HOOK_PATH="/home/gmorell/bin/gf-hook"
# Alternative examples:
# export GF_HOOK_PATH="$HOME/bin/gf-hook"
# export GF_HOOK_PATH="/usr/local/bin/gf-hook"

# Git Friends server URL (OPTIONAL)
# Default: http://localhost:8080
export GF_SERVER_URL="http://localhost:8080"
# Alternative examples:
# export GF_SERVER_URL="https://git-friends.example.com"
# export GF_SERVER_URL="http://192.168.1.100:8080"

# Authentication token (OPTIONAL)
# Only needed if your server requires authentication
# Get this token from your Git Friends server admin or generate one with:
# gf-server --generate-token yourusername
export GF_TOKEN="fo1S_usdTvG6_tYlXTQegw"
# Alternative: Leave empty if authentication is not required
# export GF_TOKEN=""

# Path to git-friends config file (OPTIONAL)
# If not set, gf-hook will look for config in standard locations
# export GF_CONFIG="$HOME/.config/git-friends.toml"
# export GF_CONFIG="/etc/git-friends.toml"

# =============================================================================
# Git Friends Environment Variables (Advanced)
# =============================================================================

# These environment variables are also supported by gf-hook directly:

# Alternative way to set the token (used by gf-hook if --token is not provided)
export GIT_FRIENDS_TOKEN="$GF_TOKEN"

# Alternative way to set the config file path
# export GIT_FRIENDS_CONFIG="$HOME/.config/git-friends.toml"

# =============================================================================
# Debugging
# =============================================================================

# Enable verbose logging for debugging
# export RUST_LOG="info"
# export RUST_LOG="debug"

# =============================================================================
# Helper Functions
# =============================================================================

# Function to test if Git Friends is properly configured
test_git_friends() {
    echo "Testing Git Friends configuration..."
    
    if [ -z "$GF_HOOK_PATH" ]; then
        echo "❌ GF_HOOK_PATH is not set"
        return 1
    fi
    
    if [ ! -x "$GF_HOOK_PATH" ]; then
        echo "❌ gf-hook binary not found or not executable at: $GF_HOOK_PATH"
        return 1
    fi
    
    echo "✅ gf-hook binary found at: $GF_HOOK_PATH"
    echo "✅ Server URL: ${GF_SERVER_URL:-http://localhost:8080}"
    
    if [ -n "$GF_TOKEN" ]; then
        echo "✅ Authentication token is set"
    else
        echo "ℹ️  No authentication token set (may be required depending on server config)"
    fi
    
    echo ""
    echo "Testing dry-run..."
    "$GF_HOOK_PATH" --server-url "${GF_SERVER_URL:-http://localhost:8080}" \
        ${GF_TOKEN:+--token "$GF_TOKEN"} \
        --dry-run
}

# Function to install the hook in the current repository
install_git_friends_hook() {
    if [ ! -d ".git" ]; then
        echo "❌ Not in a git repository"
        return 1
    fi
    
    local hook_file=".git/hooks/post-commit"
    local example_file="$(dirname "$0")/example-post-commit-environ"
    
    if [ ! -f "$example_file" ]; then
        echo "❌ example-post-commit-environ file not found"
        echo "Make sure you're running this from the git-friends directory"
        return 1
    fi
    
    cp "$example_file" "$hook_file"
    chmod +x "$hook_file"
    echo "✅ Git Friends post-commit hook installed in current repository"
    echo "Hook installed at: $hook_file"
}

# Show current Git Friends configuration
show_git_friends_config() {
    echo "Current Git Friends Configuration:"
    echo "================================="
    echo "GF_HOOK_PATH: ${GF_HOOK_PATH:-<not set>}"
    echo "GF_SERVER_URL: ${GF_SERVER_URL:-<not set>}"
    echo "GF_TOKEN: ${GF_TOKEN:+<set>}${GF_TOKEN:-<not set>}"
    echo "GF_CONFIG: ${GF_CONFIG:-<not set>}"
    echo ""
    echo "Git Friends environment variables:"
    echo "GIT_FRIENDS_TOKEN: ${GIT_FRIENDS_TOKEN:+<set>}${GIT_FRIENDS_TOKEN:-<not set>}"
    echo "GIT_FRIENDS_CONFIG: ${GIT_FRIENDS_CONFIG:-<not set>}"
    echo "RUST_LOG: ${RUST_LOG:-<not set>}"
}
