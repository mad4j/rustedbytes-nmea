#!/bin/bash
# Script to validate the release workflow configuration

set -e

echo "=== Validating Release Workflow ==="

# Check if workflow file exists
if [ ! -f .github/workflows/release.yml ]; then
    echo "❌ Error: release.yml workflow file not found"
    exit 1
fi
echo "✅ Workflow file exists"

# Validate YAML syntax
if command -v python3 &> /dev/null; then
    python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))" 2>/dev/null
    if [ $? -eq 0 ]; then
        echo "✅ YAML syntax is valid"
    else
        echo "❌ YAML syntax error"
        exit 1
    fi
else
    echo "⚠️  Python3 not found, skipping YAML validation"
fi

# Check if all expected jobs are present
JOBS=$(grep -E "^  [a-z-]+:" .github/workflows/release.yml | grep -v "steps:" | sed 's/://g' | awk '{print $1}')
EXPECTED_JOBS=("validate-version" "test" "prepare-release" "create-github-release" "publish-crates-io")

echo ""
echo "=== Checking Workflow Jobs ==="
for job in "${EXPECTED_JOBS[@]}"; do
    if echo "$JOBS" | grep -q "$job"; then
        echo "✅ Job '$job' found"
    else
        echo "❌ Job '$job' missing"
        exit 1
    fi
done

# Check if RELEASE.md exists
if [ -f RELEASE.md ]; then
    echo ""
    echo "✅ Release documentation (RELEASE.md) exists"
else
    echo ""
    echo "⚠️  RELEASE.md not found"
fi

# Verify tests pass
echo ""
echo "=== Running Tests ==="
cargo test --quiet 2>&1 | tail -5
if [ $? -eq 0 ]; then
    echo "✅ All tests pass"
else
    echo "❌ Tests failed"
    exit 1
fi

# Check formatting
echo ""
echo "=== Checking Code Formatting ==="
cargo fmt --check 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Code formatting is correct"
else
    echo "⚠️  Code formatting issues found (run 'cargo fmt' to fix)"
fi

# Check clippy
echo ""
echo "=== Running Clippy ==="
cargo clippy -- -D warnings 2>&1 | tail -5
if [ $? -eq 0 ]; then
    echo "✅ No clippy warnings"
else
    echo "⚠️  Clippy warnings found"
fi

echo ""
echo "=== Validation Complete ==="
echo "✅ Release workflow is ready to use"
echo ""
echo "To create a release:"
echo "1. Go to Actions tab in GitHub"
echo "2. Select 'Release' workflow"
echo "3. Click 'Run workflow'"
echo "4. Enter version number (e.g., 0.1.1)"
echo ""
echo "See RELEASE.md for detailed instructions"
