#!/bin/bash
# Verify B2 compound generic bounds (T: A + B syntax)
set -e

echo "Testing compound generic bounds..."

# Test valid compound bounds
cat > /tmp/compound_valid.zp << 'EOF'
fn bounded<T: number + text>(value: T) -> T:
    return value

let result: number = bounded(1)
EOF

cargo run --quiet --bin zap -- bootstrap/b1/parser.zp bootstrap/b2/typecheck.zp /tmp/compound_valid.zp /tmp/compound_valid.zp > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Valid compound bounds accepted"
else
    echo "✗ Valid compound bounds rejected"
    exit 1
fi

# Test invalid compound bounds
cat > /tmp/compound_invalid.zp << 'EOF'
fn bounded<T: number + text>(value: T) -> T:
    return value

let result: bool = bounded(true)
EOF

cargo run --quiet --bin zap -- bootstrap/b1/parser.zp bootstrap/b2/typecheck.zp /tmp/compound_invalid.zp /tmp/compound_invalid.zp > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "✓ Invalid compound bounds rejected"
else
    echo "✗ Invalid compound bounds accepted"
    exit 1
fi

echo "Compound bounds verification passed"
