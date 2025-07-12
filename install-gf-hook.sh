#!/bin/bash

# Git Friends Installation Script
# This script creates a sample git hook for the gf-hook binary

set -e

# Default values
GF_SERVER_URL="http://localhost:8080"
GF_TOKEN=""
GF_HOOK_PATH=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --server-url)
            GF_SERVER_URL="$2"
            shift 2
            ;;
        --token)
            GF_TOKEN="$2"
            shift 2
            ;;
        --hook-path)
            GF_HOOK_PATH="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --server-url URL   Git Friends server URL (default: http://localhost:8080)"
            echo "  --token TOKEN      Authentication token"
            echo "  --hook-path PATH   Path to gf-hook binary (default: ./target/release/gf-hook)"
            echo "  -h, --help         Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Set default hook path if not provided
if [ -z "$GF_HOOK_PATH" ]; then
    if [ -f "./target/release/gf-hook" ]; then
        GF_HOOK_PATH="$(pwd)/target/release/gf-hook"
    else
        echo "Error: gf-hook binary not found. Please build the project first with 'cargo build --release'"
        exit 1
    fi
fi

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "Error: Not in a git repository. Please run this script from the root of a git repository."
    exit 1
fi

# Create the post-commit hook
HOOK_FILE=".git/hooks/post-commit"

echo "Creating git hook at $HOOK_FILE"

cat > "$HOOK_FILE" << EOF
#!/bin/bash
# Git Friends post-commit hook
# This hook sends commit information to the gf-server

# Path to the gf-hook binary
GF_HOOK_PATH="$GF_HOOK_PATH"

# Server configuration
GF_SERVER_URL="$GF_SERVER_URL"
GF_TOKEN="$GF_TOKEN"

# Check if gf-hook binary exists
if [ ! -f "\$GF_HOOK_PATH" ]; then
    echo "Warning: gf-hook binary not found at \$GF_HOOK_PATH"
    exit 0
fi

# Send the commit information
if [ -n "\$GF_TOKEN" ]; then
    "\$GF_HOOK_PATH" --server-url "\$GF_SERVER_URL" --token "\$GF_TOKEN"
else
    "\$GF_HOOK_PATH" --server-url "\$GF_SERVER_URL"
fi
EOF

# Make the hook executable
chmod +x "$HOOK_FILE"

echo "Git hook created successfully!"
echo "Configuration:"
echo "  Hook file: $HOOK_FILE"
echo "  Server URL: $GF_SERVER_URL"
echo "  Token: ${GF_TOKEN:-"(none - will use environment variable GIT_FRIENDS_TOKEN)"}"
echo "  gf-hook path: $GF_HOOK_PATH"
echo ""
echo "To test the hook, make a commit in this repository."
echo "To view hook logs, set RUST_LOG=debug environment variable."
