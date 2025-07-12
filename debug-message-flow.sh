#!/bin/bash

echo "=== Git Friends Message Flow Debug ==="
echo

# Check if all components are running
echo "1. Checking component status..."

# Check server
if curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo "   ✓ gf-server is running"
else
    echo "   ✗ gf-server is not running"
    echo "   💡 Start with: ./target/release/gf-server --config test-config.toml"
    exit 1
fi

# Check MQTT broker
if command -v mosquitto_pub > /dev/null 2>&1; then
    echo "   ✓ MQTT tools available"
    timeout 2 mosquitto_pub -h localhost -p 1883 -t "test/topic" -m "test" 2>/dev/null
    if [ $? -eq 0 ]; then
        echo "   ✓ MQTT broker is reachable"
    else
        echo "   ✗ MQTT broker is not reachable"
        echo "   💡 Install and start mosquitto: sudo apt install mosquitto mosquitto-clients"
        echo "   💡 Or start with: mosquitto -v"
    fi
else
    echo "   ? MQTT tools not available (install mosquitto-clients to test)"
fi

echo
echo "2. Testing hook → server flow..."

# Test hook with server
echo "   Testing hook with actual server..."
RUST_LOG=info ./target/release/gf-hook --dry-run > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "   ✓ Hook works in dry-run mode"
    
    # Test actual send to server
    echo "   Sending actual request to server..."
    RUST_LOG=info ./target/release/gf-hook 2>&1 | grep -q "Successfully sent"
    if [ $? -eq 0 ]; then
        echo "   ✓ Hook successfully sent to server"
    else
        echo "   ✗ Hook failed to send to server"
        echo "   💡 Check server logs for errors"
    fi
else
    echo "   ✗ Hook fails in dry-run mode"
fi

echo
echo "3. Testing MQTT message flow..."

# Check if we can subscribe to MQTT topics
if command -v mosquitto_sub > /dev/null 2>&1; then
    echo "   Subscribing to MQTT topics for 5 seconds..."
    timeout 5 mosquitto_sub -h localhost -p 1883 -t "git-friends/+/+/+" -t "git-friends-test/+/+/+" &
    MQTT_PID=$!
    
    sleep 1
    
    echo "   Sending hook message..."
    ./target/release/gf-hook > /dev/null 2>&1
    
    wait $MQTT_PID
    echo "   (Check above for any MQTT messages)"
else
    echo "   ? Cannot test MQTT directly (mosquitto-clients not available)"
fi

echo
echo "4. Configuration check..."

echo "   Current config files:"
if [ -f "test-config.toml" ]; then
    echo "   ✓ test-config.toml exists"
    echo "   MQTT topic prefix: $(grep topic_prefix test-config.toml | cut -d'"' -f2)"
else
    echo "   ✗ test-config.toml missing"
fi

if [ -f "git-friends.toml" ]; then
    echo "   ✓ git-friends.toml exists"
else
    echo "   - git-friends.toml not found (using defaults)"
fi

echo
echo "5. Component integration test..."

echo "   Testing full flow with verbose logging..."
echo "   (This will show detailed logs from all components)"
echo
echo "   You should run in separate terminals:"
echo "   Terminal 1: RUST_LOG=info ./target/release/gf-server --config test-config.toml"
echo "   Terminal 2: RUST_LOG=info ./target/release/gf-irc --config test-config.toml"
echo "   Terminal 3: ./target/release/gf-hook"
echo
echo "   Or use the automated test below..."

echo
echo "=== Debug Complete ==="
echo
echo "Common issues:"
echo "1. MQTT broker not running (mosquitto)"
echo "2. Components using different topic prefixes"
echo "3. IRC client not subscribed to correct topics"
echo "4. Server not publishing to MQTT"
echo "5. Network connectivity issues"
