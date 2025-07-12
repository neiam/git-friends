# Git Friends Podman Quadlet Setup

This directory contains Podman Quadlet files for running Git Friends as systemd user services. Quadlet provides a native systemd integration for managing containers with Podman.

## Prerequisites

- Podman 4.4+ (with Quadlet support)
- systemd user services enabled
- A user account (don't run as root)

## Quick Start

1. **Complete setup** (recommended for first-time users):
   ```bash
   ./quadlet-manage.sh setup dev
   ```

2. **Manual setup**:
   ```bash
   # Build the container image
   ./quadlet-manage.sh build
   
   # Install Quadlet files
   ./quadlet-manage.sh install
   
   # Enable and start services
   ./quadlet-manage.sh enable dev
   ./quadlet-manage.sh start dev
   ```

## Available Services

### Development Profile (`dev`)
- **git-friends-mqtt**: MQTT broker (Mosquitto)
- **git-friends-server**: Main Git Friends server
- **git-friends-irc**: IRC client component

### Production Profile (`prod`)
- **git-friends-mqtt-prod**: MQTT broker with authentication
- **git-friends-server-prod**: Production server configuration
- **git-friends-irc-prod**: Production IRC client

### Testing Profile (`test`)
- **git-friends-mqtt**: MQTT broker
- **git-friends-server**: Main server
- **git-friends-tester**: Continuous testing component

## Management Commands

### Service Management
```bash
# Start services
./quadlet-manage.sh start dev     # Start development services
./quadlet-manage.sh start prod    # Start production services
./quadlet-manage.sh start test    # Start testing services

# Stop services
./quadlet-manage.sh stop dev      # Stop development services
./quadlet-manage.sh stop prod     # Stop production services
./quadlet-manage.sh stop all      # Stop all services

# Restart services
./quadlet-manage.sh restart dev   # Restart development services
```

### Service Status and Logs
```bash
# Check service status
./quadlet-manage.sh status

# View logs
./quadlet-manage.sh logs git-friends-server
./quadlet-manage.sh logs git-friends-mqtt
./quadlet-manage.sh logs git-friends-irc
```

### Enable/Disable Services
```bash
# Enable services to start on boot
./quadlet-manage.sh enable dev
./quadlet-manage.sh enable prod

# Disable all services
./quadlet-manage.sh disable
```

## Manual systemd Commands

Once installed, you can also manage services directly with systemctl:

```bash
# Start/stop individual services
systemctl --user start git-friends-server.service
systemctl --user stop git-friends-server.service

# Enable/disable services
systemctl --user enable git-friends-server.service
systemctl --user disable git-friends-server.service

# View status
systemctl --user status git-friends-server.service

# View logs
journalctl --user -u git-friends-server.service -f
```

## Quadlet Files

The `quadlet/` directory contains the following files:

### Networks
- `git-friends-network.network`: Container network for all services

### Volumes
- `mosquitto-data.volume`: Persistent storage for MQTT broker data
- `mosquitto-log.volume`: Persistent storage for MQTT broker logs
- `git-friends-logs.volume`: Persistent storage for application logs

### Containers
- `git-friends-mqtt.container`: MQTT broker (development)
- `git-friends-mqtt-prod.container`: MQTT broker (production)
- `git-friends-server.container`: Main server (development)
- `git-friends-server-prod.container`: Main server (production)
- `git-friends-irc.container`: IRC client (development)
- `git-friends-irc-prod.container`: IRC client (production)
- `git-friends-tester.container`: Testing component

## Configuration

### Volume Mounts
The Quadlet files use `%i` which resolves to the current working directory. This allows the containers to access:
- Configuration files from `./docker/config/`
- Mosquitto configuration from `./docker/mosquitto/`

### Environment Variables
- `RUST_LOG=info`: Set log level for Rust applications
- `GIT_FRIENDS_CONFIG`: Path to configuration file inside container

### Networking
All services are connected to the `git-friends-network` network, allowing them to communicate with each other using container names.

## Port Mappings

- **8080**: Git Friends server HTTP API
- **1883**: MQTT broker (unencrypted)
- **9001**: MQTT broker WebSocket (development only)

## Health Checks

The server containers include health checks that monitor the `/health` endpoint every 30 seconds.

## Troubleshooting

### Services won't start
1. Check if Podman user service is running:
   ```bash
   systemctl --user status podman.socket
   ```

2. Enable if needed:
   ```bash
   systemctl --user enable --now podman.socket
   ```

### Configuration issues
1. Ensure configuration files exist:
   ```bash
   ls -la docker/config/
   ls -la docker/mosquitto/
   ```

2. Check volume mounts in the Quadlet files point to correct paths

### View detailed logs
```bash
journalctl --user -u git-friends-server.service --no-pager
```

### SELinux issues
If you encounter SELinux-related permission issues with volume mounts, the `:Z` flag in the volume mounts should handle this automatically.

## Advantages of Quadlet over Docker Compose

1. **Native systemd integration**: Services integrate with systemd logging, dependency management, and service control
2. **User services**: Run as regular user without root privileges
3. **Better resource management**: Systemd handles resource limits and cgroups
4. **Service dependencies**: Proper dependency ordering and restart handling
5. **Logging**: Integrated with journald for centralized log management
6. **Security**: Better isolation and security with user namespaces

## Migration from Docker Compose

If you're migrating from Docker Compose:

1. Stop existing Docker Compose services:
   ```bash
   docker-compose down
   ```

2. Set up Quadlet:
   ```bash
   ./quadlet-manage.sh setup dev
   ```

3. Verify services are running:
   ```bash
   ./quadlet-manage.sh status
   ```

The Quadlet setup provides equivalent functionality to the Docker Compose setup with better systemd integration.
