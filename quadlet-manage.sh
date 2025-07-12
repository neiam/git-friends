#!/bin/bash

# Git Friends Podman Quadlet Management Script
# This script helps manage the Git Friends services using Podman Quadlet

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
QUADLET_DIR="$SCRIPT_DIR/quadlet"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        print_status $RED "This script should not be run as root. Run as a regular user."
        exit 1
    fi
}

# Function to check if podman is installed
check_podman() {
    if ! command -v podman &> /dev/null; then
        print_status $RED "Podman is not installed. Please install podman first."
        exit 1
    fi
}

# Function to check if systemd user service is enabled
check_systemd_user() {
    if ! systemctl --user status podman.socket &> /dev/null; then
        print_status $YELLOW "Podman user service is not running. Starting it..."
        systemctl --user enable --now podman.socket
    fi
}

# Function to copy quadlet files to user directory
install_quadlets() {
    local target_dir="$HOME/.config/containers/systemd"
    
    print_status $BLUE "Installing Quadlet files to $target_dir..."
    
    # Create directory if it doesn't exist
    mkdir -p "$target_dir"
    
    # Copy all quadlet files
    cp "$QUADLET_DIR"/*.{container,network,volume} "$target_dir/" 2>/dev/null || true
    
    print_status $GREEN "Quadlet files installed successfully!"
}

# Function to reload systemd daemon
reload_systemd() {
    print_status $BLUE "Reloading systemd daemon..."
    systemctl --user daemon-reload
    print_status $GREEN "Systemd daemon reloaded!"
}

# Function to build the git-friends image
build_image() {
    print_status $BLUE "Building git-friends Docker image..."
    podman build -t git-friends:latest "$SCRIPT_DIR"
    print_status $GREEN "Git-friends image built successfully!"
}

# Function to start services
start_services() {
    local profile=${1:-"dev"}
    
    print_status $BLUE "Starting Git Friends services ($profile profile)..."
    
    case $profile in
        "dev")
            systemctl --user start git-friends-network.service
            systemctl --user start git-friends-mqtt.service
            systemctl --user start git-friends-server.service
            systemctl --user start git-friends-irc.service
            ;;
        "prod")
            systemctl --user start git-friends-network.service
            systemctl --user start git-friends-mqtt-prod.service
            systemctl --user start git-friends-server-prod.service
            systemctl --user start git-friends-irc-prod.service
            ;;
        "test")
            systemctl --user start git-friends-network.service
            systemctl --user start git-friends-mqtt.service
            systemctl --user start git-friends-server.service
            systemctl --user start git-friends-tester.service
            ;;
        *)
            print_status $RED "Unknown profile: $profile. Use 'dev', 'prod', or 'test'."
            exit 1
            ;;
    esac
    
    print_status $GREEN "Services started successfully!"
}

# Function to stop services
stop_services() {
    local profile=${1:-"all"}
    
    print_status $BLUE "Stopping Git Friends services..."
    
    case $profile in
        "dev")
            systemctl --user stop git-friends-irc.service || true
            systemctl --user stop git-friends-server.service || true
            systemctl --user stop git-friends-mqtt.service || true
            ;;
        "prod")
            systemctl --user stop git-friends-irc-prod.service || true
            systemctl --user stop git-friends-server-prod.service || true
            systemctl --user stop git-friends-mqtt-prod.service || true
            ;;
        "test")
            systemctl --user stop git-friends-tester.service || true
            systemctl --user stop git-friends-server.service || true
            systemctl --user stop git-friends-mqtt.service || true
            ;;
        "all")
            systemctl --user stop git-friends-irc.service || true
            systemctl --user stop git-friends-irc-prod.service || true
            systemctl --user stop git-friends-server.service || true
            systemctl --user stop git-friends-server-prod.service || true
            systemctl --user stop git-friends-tester.service || true
            systemctl --user stop git-friends-mqtt.service || true
            systemctl --user stop git-friends-mqtt-prod.service || true
            ;;
        *)
            print_status $RED "Unknown profile: $profile. Use 'dev', 'prod', 'test', or 'all'."
            exit 1
            ;;
    esac
    
    print_status $GREEN "Services stopped successfully!"
}

# Function to show status
show_status() {
    print_status $BLUE "Git Friends Service Status:"
    echo
    
    services=(
        "git-friends-network"
        "git-friends-mqtt"
        "git-friends-mqtt-prod"
        "git-friends-server"
        "git-friends-server-prod"
        "git-friends-irc"
        "git-friends-irc-prod"
        "git-friends-tester"
    )
    
    for service in "${services[@]}"; do
        if systemctl --user is-active "$service.service" &> /dev/null; then
            print_status $GREEN "✓ $service: active"
        else
            print_status $RED "✗ $service: inactive"
        fi
    done
}

# Function to show logs
show_logs() {
    local service=${1:-"git-friends-server"}
    print_status $BLUE "Showing logs for $service..."
    journalctl --user -u "$service.service" -f
}

# Function to enable services
enable_services() {
    local profile=${1:-"dev"}
    
    print_status $BLUE "Enabling Git Friends services ($profile profile)..."
    
    systemctl --user enable git-friends-network.service
    
    case $profile in
        "dev")
            systemctl --user enable git-friends-mqtt.service
            systemctl --user enable git-friends-server.service
            systemctl --user enable git-friends-irc.service
            ;;
        "prod")
            systemctl --user enable git-friends-mqtt-prod.service
            systemctl --user enable git-friends-server-prod.service
            systemctl --user enable git-friends-irc-prod.service
            ;;
        "test")
            systemctl --user enable git-friends-mqtt.service
            systemctl --user enable git-friends-server.service
            systemctl --user enable git-friends-tester.service
            ;;
        *)
            print_status $RED "Unknown profile: $profile. Use 'dev', 'prod', or 'test'."
            exit 1
            ;;
    esac
    
    print_status $GREEN "Services enabled successfully!"
}

# Function to disable services
disable_services() {
    print_status $BLUE "Disabling Git Friends services..."
    
    services=(
        "git-friends-network"
        "git-friends-mqtt"
        "git-friends-mqtt-prod"
        "git-friends-server"
        "git-friends-server-prod"
        "git-friends-irc"
        "git-friends-irc-prod"
        "git-friends-tester"
    )
    
    for service in "${services[@]}"; do
        systemctl --user disable "$service.service" || true
    done
    
    print_status $GREEN "Services disabled successfully!"
}

# Main function
main() {
    check_root
    check_podman
    
    case "${1:-help}" in
        "install")
            check_systemd_user
            install_quadlets
            reload_systemd
            ;;
        "build")
            build_image
            ;;
        "start")
            start_services "${2:-dev}"
            ;;
        "stop")
            stop_services "${2:-all}"
            ;;
        "restart")
            stop_services "${2:-all}"
            start_services "${2:-dev}"
            ;;
        "status")
            show_status
            ;;
        "logs")
            show_logs "${2:-git-friends-server}"
            ;;
        "enable")
            enable_services "${2:-dev}"
            ;;
        "disable")
            disable_services
            ;;
        "setup")
            check_systemd_user
            build_image
            install_quadlets
            reload_systemd
            enable_services "${2:-dev}"
            start_services "${2:-dev}"
            ;;
        "help"|*)
            echo "Git Friends Podman Quadlet Management Script"
            echo
            echo "Usage: $0 <command> [options]"
            echo
            echo "Commands:"
            echo "  install           Install Quadlet files to user systemd directory"
            echo "  build             Build the git-friends Docker image"
            echo "  start [profile]   Start services (dev|prod|test, default: dev)"
            echo "  stop [profile]    Stop services (dev|prod|test|all, default: all)"
            echo "  restart [profile] Restart services (dev|prod|test, default: dev)"
            echo "  status            Show service status"
            echo "  logs [service]    Show logs for service (default: git-friends-server)"
            echo "  enable [profile]  Enable services to start on boot (dev|prod|test, default: dev)"
            echo "  disable           Disable all services"
            echo "  setup [profile]   Complete setup (build, install, enable, start)"
            echo "  help              Show this help message"
            echo
            echo "Examples:"
            echo "  $0 setup dev      # Complete development setup"
            echo "  $0 setup prod     # Complete production setup"
            echo "  $0 start dev      # Start development services"
            echo "  $0 stop all       # Stop all services"
            echo "  $0 logs git-friends-server  # Show server logs"
            ;;
    esac
}

main "$@"
