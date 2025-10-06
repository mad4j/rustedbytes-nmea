#!/bin/bash
# Update test count badge in README.md

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <test_count>"
    exit 1
fi

TEST_COUNT=$1
README_FILE="README.md"

# Update the badge line using sed
# Match the test count badge line and replace the number
sed -i "s/\[\!\[Test Count\](https:\/\/img\.shields\.io\/badge\/tests-[0-9]\+-brightgreen\.svg)\]/\[\!\[Test Count\](https:\/\/img.shields.io\/badge\/tests-${TEST_COUNT}-brightgreen.svg)\]/" "$README_FILE"

echo "Updated test count badge to ${TEST_COUNT}"
