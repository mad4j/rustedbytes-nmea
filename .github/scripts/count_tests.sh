#!/bin/bash
# Count total number of tests from cargo test output

set -e

# Run cargo test and capture output
TEST_OUTPUT=$(cargo test 2>&1)

# Extract test counts from "running X tests" lines and sum them
TOTAL_TESTS=$(echo "$TEST_OUTPUT" | grep -E "^running [0-9]+ tests" | awk '{sum += $2} END {print sum}')

echo "$TOTAL_TESTS"
