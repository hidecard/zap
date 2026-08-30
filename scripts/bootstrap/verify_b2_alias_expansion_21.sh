#!/bin/bash
# Verify B2 alias expansion and validation
set -e

echo "Testing alias expansion..."

# Test nested alias expansion
cat > /tmp/alias_nested.zp << 'EOF'
type IntList = list<number>

let values: IntList = [1, 2, 3]
EOF

cargo run --quiet --bin zap -- bootstrap/b1/parser.zp bootstrap/b2/typecheck.zp /tmp/alias_nested.zp /tmp/alias_nested.zp > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Nested alias expansion accepted"
else
    echo "✗ Nested alias expansion rejected"
    exit 1
fi

# Test alias of alias
cat > /tmp/alias_of_alias.zp << 'EOF'
type Inner = list<number>
type Outer = Inner

let values: Outer = [1, 2, 3]
EOF

cargo run --quiet --bin zap -- bootstrap/b1/parser.zp bootstrap/b2/typecheck.zp /tmp/alias_of_alias.zp /tmp/alias_of_alias.zp > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Alias of alias accepted"
else
    echo "✗ Alias of alias rejected"
    exit 1
fi

# Test generic alias
cat > /tmp/alias_generic.zp << 'EOF'
type Box<T> = option<T>

let boxed: Box<number> = some(42)
EOF

cargo run --quiet --bin zap -- bootstrap/b1/parser.zp bootstrap/b2/typecheck.zp /tmp/alias_generic.zp /tmp/alias_generic.zp > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Generic alias accepted"
else
    echo "✗ Generic alias rejected"
    exit 1
fi

# Test undeclared parameter
cat > /tmp/alias_undeclared.zp << 'EOF'
type Box<T> = option<U>

let boxed: Box<number> = some(42)
EOF

cargo run --quiet --bin zap -- bootstrap/b1/parser.zp bootstrap/b2/typecheck.zp /tmp/alias_undeclared.zp /tmp/alias_undeclared.zp > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "✓ Undeclared parameter rejected"
else
    echo "✗ Undeclared parameter accepted"
    exit 1
fi

echo "Alias expansion verification passed"
