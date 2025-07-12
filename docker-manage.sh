#!/bin/bash
# Git Friends Docker Management Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default environment
ENVIRONMENT="dev"
COMPOSE_FILE="docker-compose.yml"

# Function to print usage
usage() {
    echo "Git Friends Docker Management Script"
    echo "Usage: $0 [ENVIRONMENT] [COMMAND] [OPTIONS]"
    echo
    echo "Environments:"
    echo "  dev        Development environment (default)"
    echo "  prod       Production environment"
    echo
    echo "Commands:"
    echo "  build      Build the Docker images"
    echo "  up         Start the services"
    echo "  down       Stop the services"
    echo "  restart    Restart the services"
    echo "  logs       View logs"
    echo "  status     Show service status"
    echo "  test       Run the tester"
    echo "  shell      Open shell in a container"
    echo "  token      Generate authentication token"
    echo "  clean      Clean up containers and volumes"
    echo
    echo "Options:"
    echo "  -d, --detach    Run in detached mode"
    echo "  -f, --follow    Follow log output"
    echo "  --build         Force rebuild when starting"
    echo
    echo "Examples:"
    echo "  $0 dev up -d              # Start development environment in background"
    echo "  $0 prod build             # Build production images"
    echo "  $0 logs --follow          # Follow development logs"
    echo "  $0 dev test               # Run tester in development"
    echo "  $0 prod token myuser      # Generate token for production"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        dev|prod)
            ENVIRONMENT=$1
            if [ "$ENVIRONMENT" = "prod" ]; then
                COMPOSE_FILE="docker-compose.prod.yml"
            fi
            shift
            ;;
        build|up|down|restart|logs|status|test|shell|token|clean)
            COMMAND=$1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            break
            ;;
    esac
done

# Set default command if not provided
if [ -z "$COMMAND" ]; then
    COMMAND="status"
fi

# Docker Compose command prefix
DC="docker-compose -f $COMPOSE_FILE"

echo -e "${BLUE}Git Friends Docker Manager${NC}"
echo -e "Environment: ${YELLOW}$ENVIRONMENT${NC}"
echo -e "Compose file: ${YELLOW}$COMPOSE_FILE${NC}"
echo

# Execute commands
case $COMMAND in
    build)
        echo -e "${BLUE}Building Docker images...${NC}"
        $DC build "$@"
        ;;
    
    up)
        echo -e "${BLUE}Starting services...${NC}"
        $DC up "$@"
        ;;
    
    down)
        echo -e "${BLUE}Stopping services...${NC}"
        $DC down "$@"
        ;;
    
    restart)
        echo -e "${BLUE}Restarting services...${NC}"
        $DC restart "$@"
        ;;
    
    logs)
        echo -e "${BLUE}Viewing logs...${NC}"
        $DC logs "$@"
        ;;
    
    status)
        echo -e "${BLUE}Service status:${NC}"
        $DC ps
        echo
        echo -e "${BLUE}Health checks:${NC}"
        docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep git-friends || echo "No Git Friends containers running"
        ;;
    
    test)
        echo -e "${BLUE}Running tester...${NC}"
        if [ "$ENVIRONMENT" = "dev" ]; then
            $DC --profile testing up git-friends-tester
        else
            echo -e "${YELLOW}Tester not available in production environment${NC}"
        fi
        ;;
    
    shell)
        SERVICE=${1:-git-friends-server}
        echo -e "${BLUE}Opening shell in $SERVICE...${NC}"
        $DC exec "$SERVICE" /bin/bash
        ;;
    
    token)
        USERNAME=${1:-"user"}
        echo -e "${BLUE}Generating token for user: $USERNAME${NC}"
        $DC exec git-friends-server ./bin/gf-server --generate-token "$USERNAME"
        ;;
    
    clean)
        echo -e "${YELLOW}Cleaning up containers and volumes...${NC}"
        $DC down -v
        docker system prune -f
        echo -e "${GREEN}Cleanup complete${NC}"
        ;;
    
    *)
        echo -e "${RED}Unknown command: $COMMAND${NC}"
        usage
        exit 1
        ;;
esac
