#!/bin/bash
# Test code examples from the mdBook guide

set -e

echo "Testing code examples from the book..."
echo ""

# Create temp directory for test files
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Counter for tests
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Function to extract and test a code block
test_code_block() {
    local file=$1
    local block_num=$2
    local code=$3

    TESTS_RUN=$((TESTS_RUN + 1))

    # Create test file
    local test_file="$TEMP_DIR/test_${TESTS_RUN}.rs"

    # Wrap in main if needed
    if echo "$code" | grep -q "fn main"; then
        echo "$code" > "$test_file"
    else
        # It's a snippet - wrap it
        cat > "$test_file" <<EOF
use tunes::prelude::*;
use tunes::sequences;

fn main() -> anyhow::Result<()> {
$code
    Ok(())
}
EOF
    fi

    # Try to compile
    echo -n "Testing example $TESTS_RUN from $file:$block_num... "

    if rustc --edition 2021 --crate-type bin \
        -L ../target/release/deps \
        --extern tunes=../target/release/libtunes.rlib \
        --extern anyhow=../target/release/deps/libanyhow-*.rlib \
        "$test_file" -o "$TEMP_DIR/test_${TESTS_RUN}" 2>/dev/null; then
        echo "✓ PASS"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo "✗ FAIL"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        # Show the code that failed
        echo "  Code:"
        sed 's/^/    /' "$test_file"
    fi
}

# Extract code blocks from a markdown file
extract_and_test() {
    local md_file=$1

    if [ ! -f "$md_file" ]; then
        return
    fi

    echo ""
    echo "=== Processing $md_file ==="

    # Simple extraction (assumes ```rust blocks)
    # This is a basic implementation - could be more robust
    awk '
        /^```rust$/ { in_block=1; block_num++; code=""; next }
        /^```$/ && in_block {
            print "BLOCK:" block_num
            print code
            print "ENDBLOCK"
            in_block=0
            next
        }
        in_block { code = code $0 "\n" }
    ' "$md_file" | while IFS= read -r line; do
        if [[ $line == BLOCK:* ]]; then
            block_num="${line#BLOCK:}"
            code=""
        elif [[ $line == "ENDBLOCK" ]]; then
            # Test this code block (skip if empty)
            if [ -n "$code" ]; then
                test_code_block "$md_file" "$block_num" "$code"
            fi
        else
            code+="$line"$'\n'
        fi
    done
}

# First, ensure the library is built
echo "Building tunes library..."
cd ..
cargo build --release --lib 2>&1 | tail -5
cd book

# Test all markdown files
for file in src/game-audio/*.md src/getting-started/*.md src/concepts/*.md; do
    extract_and_test "$file"
done

# Summary
echo ""
echo "=============================="
echo "Test Summary:"
echo "  Total:  $TESTS_RUN"
echo "  Passed: $TESTS_PASSED"
echo "  Failed: $TESTS_FAILED"
echo "=============================="

if [ $TESTS_FAILED -gt 0 ]; then
    exit 1
fi
