# Git Friends Docker Setup

This directory contains a complete Docker setup for running Git Friends with all its components (server, IRC client, and MQTT broker).

## 🚀 Quick Start

### Development Environment

```bash
# Build and start all services
./docker-manage.sh dev build
./docker-manage.sh dev up -d

# Check status
./docker-manage.sh status

# View logs
./docker-manage.sh logs -f

# Test with the built-in tester
./docker-manage.sh test
```

### Production Environment

```bash
# Build production images
./docker-manage.sh prod build

# Start production services
./docker-manage.sh prod up -d

# Generate authentication token
./docker-manage.sh prod token myusername
```

## 📁 File Structure

```
docker/
├── config/
│   ├── git-friends.toml       # Development configuration
│   └── git-friends.prod.toml  # Production configuration
├── mosquitto/
│   ├── mosquitto.conf         # MQTT broker config
│   ├── data/                  # MQTT persistence data
│   └── log/                   # MQTT logs
└── logs/                      # Application logs

Dockerfile                     # Multi-stage build definition
docker-compose.yml             # Development environment
docker-compose.prod.yml        # Production environment
docker-manage.sh              # Management script
.dockerignore                 # Build optimization
```

## 🛠️ Services

### Git Friends Server (`git-friends-server`)
- **Port**: 8080
- **Purpose**: HTTP webhook receiver
- **Health check**: `GET /health`
- **Configuration**: Via mounted config file

### Git Friends IRC Client (`git-friends-irc`)
- **Purpose**: MQTT → IRC bridge
- **Dependencies**: Server, MQTT broker
- **Configuration**: Via mounted config file

### MQTT Broker (`mqtt`)
- **Port**: 1883 (MQTT), 9001 (WebSocket)
- **Image**: `eclipse-mosquitto:2.0`
- **Persistence**: Enabled with Docker volumes

### Tester (`git-friends-tester`)
- **Purpose**: Development testing
- **Profile**: `testing` (development only)
- **Mode**: Continuous with 30-second intervals

## 🔧 Configuration

### Development Configuration

Located at `docker/config/git-friends.toml`:

```toml
[server]
bind_address = "0.0.0.0:8080"

[mqtt]
broker_host = "mqtt"  # Docker service name
broker_port = 1883

[irc]
server = "irc.libera.chat"
channels = ["#git-friends"]
topic_filters = ["git-friends/+/+", "git-friends/+/+/+"]

[auth]
require_auth = false  # Development default
```

### Production Configuration

Copy and customize `docker/config/git-friends.prod.toml`:

```toml
[auth]
require_auth = true
[[auth.tokens]]
token = "your-secure-token-here"
username = "your-username"
```

## 🐳 Docker Management

### Management Script (`docker-manage.sh`)

```bash
# Environment commands
./docker-manage.sh [dev|prod] [command] [options]

# Build images
./docker-manage.sh dev build

# Start services
./docker-manage.sh dev up -d

# View logs
./docker-manage.sh logs -f

# Check status
./docker-manage.sh status

# Generate token
./docker-manage.sh prod token username

# Open shell
./docker-manage.sh shell git-friends-server

# Clean up
./docker-manage.sh clean
```

### Direct Docker Compose

```bash
# Development
docker-compose up -d
docker-compose logs -f
docker-compose down

# Production
docker-compose -f docker-compose.prod.yml up -d
docker-compose -f docker-compose.prod.yml logs -f
docker-compose -f docker-compose.prod.yml down
```

## 🔐 Security

### Development
- Anonymous MQTT access
- No authentication required
- All ports exposed

### Production
- Authentication required
- MQTT credentials
- Secure token generation
- Optional SSL/TLS reverse proxy

## 📊 Monitoring

### Health Checks
- Server health endpoint: `http://localhost:8080/health`
- Docker health checks every 30 seconds
- Automatic restart on failure

### Logs
- Application logs: `docker/logs/`
- MQTT logs: `docker/mosquitto/log/`
- Container logs: `docker-compose logs`

### Service Status
```bash
# Check all services
./docker-manage.sh status

# View specific service logs
docker-compose logs git-friends-server
docker-compose logs git-friends-irc
docker-compose logs mqtt
```

## 🧪 Testing

### Built-in Tester
```bash
# Run continuous tester (development only)
./docker-manage.sh test

# Manual testing
docker-compose exec git-friends-server ./bin/gf-tester --count 1
```

### External Testing
```bash
# Test webhook endpoint
curl -X POST http://localhost:8080/webhook \
  -H "Content-Type: application/json" \
  -d '{"hash":"test123","author_name":"tester","message":"test commit"}'

# Test health endpoint
curl http://localhost:8080/health
```

## 🚀 Production Deployment

### 1. Configuration
```bash
# Copy and customize production config
cp docker/config/git-friends.toml docker/config/git-friends.prod.toml

# Edit production settings
nano docker/config/git-friends.prod.toml
```

### 2. Build and Deploy
```bash
# Build production images
./docker-manage.sh prod build

# Start production services
./docker-manage.sh prod up -d

# Generate authentication tokens
./docker-manage.sh prod token user1
./docker-manage.sh prod token user2
```

### 3. Configure Git Hooks
```bash
# Update git hooks to use production server
export GF_SERVER_URL="http://your-server:8080"
export GF_TOKEN="your-generated-token"
```

## 🔧 Customization

### Environment Variables
```yaml
# In docker-compose.yml
environment:
  - RUST_LOG=debug  # Change log level
  - GIT_FRIENDS_CONFIG=/app/config/git-friends.toml
```

### Volume Mounts
```yaml
# Custom configuration
volumes:
  - ./my-config.toml:/app/config/git-friends.toml:ro
  - ./my-logs:/app/logs
```

### Network Configuration
```yaml
# External network
networks:
  default:
    external:
      name: my-network
```

## 🐛 Troubleshooting

### Common Issues

1. **MQTT Connection Failed**
   ```bash
   # Check MQTT broker logs
   docker-compose logs mqtt
   
   # Test MQTT connectivity
   docker-compose exec git-friends-server ./bin/gf-tester --count 1
   ```

2. **IRC Connection Issues**
   ```bash
   # Check IRC client logs
   docker-compose logs git-friends-irc
   
   # Verify IRC server configuration
   docker-compose exec git-friends-irc ./bin/gf-irc --help
   ```

3. **Authentication Errors**
   ```bash
   # Generate new token
   ./docker-manage.sh token myuser
   
   # Check auth configuration
   docker-compose exec git-friends-server cat /app/config/git-friends.toml
   ```

4. **Health Check Failures**
   ```bash
   # Check service status
   ./docker-manage.sh status
   
   # Manual health check
   curl http://localhost:8080/health
   ```

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug ./docker-manage.sh dev up

# View detailed logs
./docker-manage.sh logs -f
```

## 🔄 Updates

### Update Images
```bash
# Pull latest dependencies
docker-compose pull

# Rebuild application
./docker-manage.sh build

# Restart services
./docker-manage.sh restart
```

### Configuration Updates
```bash
# Edit configuration
nano docker/config/git-friends.toml

# Restart affected services
docker-compose restart git-friends-server git-friends-irc
```

## 📈 Scaling

### Horizontal Scaling
```yaml
# Scale server instances
docker-compose up -d --scale git-friends-server=3

# Load balancer required for multiple instances
```

### Resource Limits
```yaml
# In docker-compose.yml
deploy:
  resources:
    limits:
      cpus: '0.5'
      memory: 512M
```

## 🎯 Best Practices

1. **Use specific image tags** instead of `latest`
2. **Set resource limits** for production
3. **Use secrets** for sensitive configuration
4. **Enable log rotation** for long-running instances
5. **Monitor** service health and performance
6. **Backup** MQTT persistence data and logs
7. **Use SSL/TLS** for production deployments
