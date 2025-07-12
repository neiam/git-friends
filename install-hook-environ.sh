#!/bin/bash
# install-hook-environ.sh
#
# Installation script for the environment-based Git Friends post-commit hook
# This script helps set up the Git Friends hook in your repositories

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Git Friends Environment-Based Hook Installer${NC}"
echo "=============================================="
echo

# Check if we're in the git-friends directory
if [ ! -f "example-post-commit-environ" ]; then
    echo -e "${RED}❌ Error: example-post-commit-environ not found${NC}"
    echo "Please run this script from the git-friends project directory"
    exit 1
fi

# Function to install in a specific directory
install_hook_in_dir() {
    local target_dir="$1"
    
    if [ ! -d "$target_dir/.git" ]; then
        echo -e "${RED}❌ Error: $target_dir is not a git repository${NC}"
        return 1
    fi
    
    local hook_file="$target_dir/.git/hooks/post-commit"
    
    # Backup existing hook if it exists
    if [ -f "$hook_file" ]; then
        echo -e "${YELLOW}⚠️  Backing up existing post-commit hook${NC}"
        cp "$hook_file" "$hook_file.backup.$(date +%Y%m%d_%H%M%S)"
    fi
    
    # Install the new hook
    cp "example-post-commit-environ" "$hook_file"
    chmod +x "$hook_file"
    
    echo -e "${GREEN}✅ Git Friends hook installed in $target_dir${NC}"
    echo "   Hook file: $hook_file"
}

# Function to set up the profile
setup_profile() {
    local profile_target="$HOME/.git-friends.profile"
    
    if [ -f "$profile_target" ]; then
        echo -e "${YELLOW}⚠️  Profile already exists at $profile_target${NC}"
        read -p "Overwrite? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Skipping profile setup"
            return 0
        fi
    fi
    
    cp ".git-friends.profile" "$profile_target"
    echo -e "${GREEN}✅ Profile copied to $profile_target${NC}"
    
    # Update the profile with the correct hook path
    local hook_path="$(pwd)/target/release/gf-hook"
    if [ -f "$hook_path" ]; then
        sed -i "s|export GF_HOOK_PATH=\"\$HOME/bin/gf-hook\"|export GF_HOOK_PATH=\"$hook_path\"|" "$profile_target"
        echo -e "${GREEN}✅ Profile updated with correct hook path${NC}"
    else
        echo -e "${YELLOW}⚠️  gf-hook binary not found at $hook_path${NC}"
        echo "Please edit $profile_target and set the correct GF_HOOK_PATH"
    fi
}

# Function to add profile to shell config
add_to_shell_config() {
    local shell_config=""
    
    if [ -f "$HOME/.bashrc" ]; then
        shell_config="$HOME/.bashrc"
    elif [ -f "$HOME/.zshrc" ]; then
        shell_config="$HOME/.zshrc"
    else
        echo -e "${YELLOW}⚠️  Could not find shell config file${NC}"
        return 1
    fi
    
    if grep -q "git-friends.profile" "$shell_config"; then
        echo -e "${YELLOW}⚠️  Profile already sourced in $shell_config${NC}"
        return 0
    fi
    
    echo "" >> "$shell_config"
    echo "# Git Friends configuration" >> "$shell_config"
    echo "if [ -f \"\$HOME/.git-friends.profile\" ]; then" >> "$shell_config"
    echo "    source \"\$HOME/.git-friends.profile\"" >> "$shell_config"
    echo "fi" >> "$shell_config"
    
    echo -e "${GREEN}✅ Profile added to $shell_config${NC}"
}

# Parse command line arguments
INSTALL_PROFILE=true
INSTALL_HOOK=true
TARGET_DIR="."

while [[ $# -gt 0 ]]; do
    case $1 in
        --no-profile)
            INSTALL_PROFILE=false
            shift
            ;;
        --no-hook)
            INSTALL_HOOK=false
            shift
            ;;
        --target-dir)
            TARGET_DIR="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --no-profile    Don't install the .git-friends.profile"
            echo "  --no-hook       Don't install the post-commit hook"
            echo "  --target-dir    Directory to install hook in (default: current directory)"
            echo "  -h, --help      Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}❌ Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Install profile if requested
if [ "$INSTALL_PROFILE" = true ]; then
    echo -e "${BLUE}Setting up Git Friends profile...${NC}"
    setup_profile
    
    echo
    read -p "Add profile to shell config? [y/N]: " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        add_to_shell_config
    fi
    echo
fi

# Install hook if requested
if [ "$INSTALL_HOOK" = true ]; then
    echo -e "${BLUE}Installing Git Friends hook...${NC}"
    install_hook_in_dir "$TARGET_DIR"
    echo
fi

echo -e "${GREEN}🎉 Installation complete!${NC}"
echo
echo "Next steps:"
echo "1. Source the profile: source ~/.git-friends.profile"
echo "2. Test the configuration: test_git_friends"
echo "3. Make sure your gf-server is running"
echo "4. Make a commit to test the hook!"
echo
echo "Useful commands:"
echo "  test_git_friends          - Test your configuration"
echo "  show_git_friends_config   - Show current settings"
echo "  install_git_friends_hook  - Install hook in current repo"
