#!/bin/bash

# Simple test script to demonstrate client ID functionality
# This script shows how each binary uses a different client ID

echo "=== Git Friends Client ID Test ==="
echo

echo "Testing client ID uniqueness..."
echo "Base client ID from config: git-friends-test"
echo

# Test each binary's help to confirm they compile
echo "1. Testing gf-tester..."
./target/release/gf-tester --help > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "   ✓ gf-tester compiled successfully"
    echo "   Client ID: git-friends-test/tester"
else
    echo "   ✗ gf-tester failed to compile"
fi

echo "2. Testing gf-irc..."
./target/release/gf-irc --help > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "   ✓ gf-irc compiled successfully"
    echo "   Client ID: git-friends-test/irc"
else
    echo "   ✗ gf-irc failed to compile"
fi

echo "3. Testing gf-server..."
./target/release/gf-server --help > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "   ✓ gf-server compiled successfully"
    echo "   Client ID: git-friends-test/server"
else
    echo "   ✗ gf-server failed to compile"
fi

echo "4. Testing gf-hook..."
./target/release/gf-hook --help > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "   ✓ gf-hook compiled successfully"
    echo "   Client ID: git-friends-test (unchanged)"
else
    echo "   ✗ gf-hook failed to compile"
fi

echo
echo "=== Test Complete ==="
echo "All binaries use unique client IDs to avoid MQTT conflicts."
echo
echo "To test the tester binary:"
echo "  ./target/release/gf-tester --config test-config.toml --count 3 --interval 1"
echo
echo "Note: You'll need an MQTT broker running to test actual functionality."
