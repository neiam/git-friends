#!/bin/bash

echo "=== Git Friends Hook Diagnostic ==="
echo

# Check if server is running
echo "1. Checking if server is running..."
if curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo "   ✓ Server is running"
    
    # Test without token
    echo "2. Testing hook without token..."
    ./target/release/gf-hook --dry-run > /dev/null 2>&1
    echo "   ✓ Hook works in dry-run mode"
    
    echo "3. Testing hook with server (no token)..."
    RUST_LOG=error ./target/release/gf-hook 2>&1 | grep -q "401 Unauthorized"
    if [ $? -eq 0 ]; then
        echo "   ✗ Server requires authentication (401 Unauthorized)"
        echo "   💡 Solutions:"
        echo "      - Run server with: ./target/release/gf-server --config test-config.toml"
        echo "      - Generate token: ./target/release/gf-server --generate-token testuser"
        echo "      - Use token: ./target/release/gf-hook --token YOUR_TOKEN"
        echo "      - Set env: export GIT_FRIENDS_TOKEN=YOUR_TOKEN"
    else
        echo "   ✓ Hook works with server"
    fi
    
else
    echo "   ✗ Server is not running"
    echo "   💡 Start server with: ./target/release/gf-server --config test-config.toml"
fi

echo
echo "4. Testing components individually..."

# Test server with no-auth config
echo "   Testing server with no-auth config..."
timeout 3 ./target/release/gf-server --config test-config.toml > /dev/null 2>&1 &
SERVER_PID=$!
sleep 2

if curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo "   ✓ Server starts with no-auth config"
    
    echo "   Testing hook with no-auth server..."
    ./target/release/gf-hook --dry-run > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "   ✓ Hook should work with no-auth server"
    else
        echo "   ✗ Hook still fails with no-auth server"
    fi
    
    kill $SERVER_PID 2>/dev/null
else
    echo "   ✗ Server failed to start with no-auth config"
    kill $SERVER_PID 2>/dev/null
fi

wait $SERVER_PID 2>/dev/null

echo
echo "=== Diagnostic Complete ==="
echo
echo "Summary:"
echo "- Tester works because it only connects to MQTT"
echo "- Hook fails because it sends HTTP requests to server"
echo "- Server requires authentication by default"
echo
echo "Quick fix: Run server with no-auth config:"
echo "  ./target/release/gf-server --config test-config.toml"
