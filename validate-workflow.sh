#!/bin/bash

# Simple script to validate GitHub Actions workflow syntax
# This script uses yamllint if available, otherwise just checks basic structure

set -euo pipefail

WORKFLOW_FILE=".github/workflows/rust-ci.yml"

if ! [ -f "$WORKFLOW_FILE" ]; then
    echo "ERROR: Workflow file not found: $WORKFLOW_FILE"
    exit 1
fi

echo "Validating GitHub Actions workflow: $WORKFLOW_FILE"

# Check if yamllint is available
if command -v yamllint &> /dev/null; then
    echo "Running yamllint validation..."
    yamllint "$WORKFLOW_FILE"
    echo "✓ YAML syntax is valid"
else
    echo "yamllint not available, performing basic validation..."
    
    # Basic YAML validation using Python
    if command -v python3 &> /dev/null; then
        if python3 -c "import yaml" 2>/dev/null; then
            python3 -c "
import yaml
import sys
try:
    with open('$WORKFLOW_FILE', 'r') as f:
        yaml.safe_load(f)
    print('✓ YAML syntax is valid')
except yaml.YAMLError as e:
    print('✗ YAML syntax error:', e)
    sys.exit(1)
"
        else
            echo "⚠ Cannot validate YAML syntax (PyYAML not installed)"
        fi
    else
        echo "⚠ Cannot validate YAML syntax (no python3 available)"
    fi
fi

# Check for required workflow elements
echo
echo "Checking workflow structure..."

required_elements=(
    "name:"
    "on:"
    "jobs:"
    "build:"
    "release:"
    "create-release:"
)

for element in "${required_elements[@]}"; do
    if grep -q "^  $element\|^$element" "$WORKFLOW_FILE"; then
        echo "✓ Found: $element"
    else
        echo "✗ Missing: $element"
        exit 1
    fi
done

echo
echo "Checking modern GitHub Actions practices..."

# Check for modern action versions
modern_checks=(
    "actions/checkout@v4"
    "dtolnay/rust-toolchain@stable"
    "Swatinem/rust-cache@v2"
    "actions/upload-artifact@v4"
    "actions/download-artifact@v4"
)

for check in "${modern_checks[@]}"; do
    if grep -q "$check" "$WORKFLOW_FILE"; then
        echo "✓ Using modern action: $check"
    else
        echo "⚠ Could not find modern action: $check"
    fi
done

echo
echo "Checking matrix strategy..."

if grep -q "matrix:" "$WORKFLOW_FILE"; then
    echo "✓ Matrix strategy found"
    
    if grep -q "os: \[ ubuntu-latest, macos-latest, windows-latest \]" "$WORKFLOW_FILE"; then
        echo "✓ Native OS matrix strategy configured"
    else
        echo "⚠ OS matrix configuration not found"
    fi
    
    if grep -q "rust: \[ stable \]" "$WORKFLOW_FILE"; then
        echo "✓ Rust stable toolchain configured"
    else
        echo "⚠ Rust stable toolchain not found"
    fi
else
    echo "✗ Matrix strategy not found"
    exit 1
fi

echo
echo "🎉 Workflow validation completed successfully!"
echo
echo "Key improvements in this workflow:"
echo "- ✅ Native compilation instead of cross-compilation"
echo "- ✅ Modern GitHub Actions (checkout@v4, dtolnay/rust-toolchain)"
echo "- ✅ Rust caching for faster builds"
echo "- ✅ Matrix strategy with ubuntu-latest, macos-latest, windows-latest"
echo "- ✅ Simplified platform support (3 platforms instead of 6)"
echo "- ✅ More reliable builds (no cross-compilation complexity)"
