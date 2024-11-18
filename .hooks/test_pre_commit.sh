#!/bin/bash

# Enable error handling
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Project specific paths
PROJECT_ROOT="$(git rev-parse --show-toplevel)"
HOOK_PATH="$PROJECT_ROOT/.hooks/pre-commit/target/release/pre-commit"

# Build the hook
cd "$PROJECT_ROOT/.hooks/pre-commit"
cargo build --release

echo -e "${YELLOW}Checking pre-commit hook...${NC}"

# Check if hook exists and is compiled
if [ ! -f "$HOOK_PATH" ]; then
    echo -e "${RED}Error: Pre-commit hook not found at $HOOK_PATH${NC}"
    echo "Please compile the hook first:"
    echo "cd .hooks/pre-commit && cargo build --release"
    exit 1
fi

echo -e "${YELLOW}Setting up test environment...${NC}"

# Create temporary test repository
TEST_DIR=$(mktemp -d)
echo "Created test directory: $TEST_DIR"

# Initialize git repo with main branch
cd "$TEST_DIR"
git init -b main > /dev/null 2>&1

# Configure git for testing
git config --local user.name "Test User"
git config --local user.email "test@example.com"

# Create initial commit so we can test against HEAD
touch .gitkeep
git add .gitkeep
git commit -m "Initial commit" > /dev/null 2>&1

# Copy hook to test repository
mkdir -p .git/hooks
cp "$HOOK_PATH" .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

echo -e "${YELLOW}Running tests...${NC}"

# Function to run a test case
run_test() {
    local test_name=$1
    local content=$2
    local expected_exit_code=$3
    local message=$4
    local force=${5:-false}

    echo -e "\n${YELLOW}Test: ${NC}$test_name"
    echo "Force flag: $force"

    # Reset the repository to a clean state for each test
    rm -rf .git
    git init -b main > /dev/null 2>&1
    git config --local user.name "Test User"
    git config --local user.email "test@example.com"

    # Create initial commit with .gitkeep
    touch .gitkeep
    git add .gitkeep
    git commit -m "Initial commit" > /dev/null 2>&1

    # Reinstall the hook for the new repo
    mkdir -p .git/hooks
    cp "$HOOK_PATH" .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit

    # Create test file with content
    echo "$content" > test.rs

    # Stage the file
    git add test.rs

    # Debug output if DEBUG is set
    if [ "${DEBUG:-false}" = true ]; then
        echo "Git status:"
        git status
        echo "Git diff:"
        git diff --cached
    fi

    # Try to commit
    if [ "$force" = true ]; then
        echo "Attempting force commit..."
        COMMIT_MSG_FILE=$(mktemp)
        echo "test commit [force]" > "$COMMIT_MSG_FILE"

        GIT_COMMIT_FORCE=1 .git/hooks/pre-commit "$COMMIT_MSG_FILE"
        actual_exit_code=$?

        rm "$COMMIT_MSG_FILE"
    else
        COMMIT_MSG_FILE=$(mktemp)
        echo "test commit" > "$COMMIT_MSG_FILE"

        .git/hooks/pre-commit "$COMMIT_MSG_FILE"
        actual_exit_code=$?

        rm "$COMMIT_MSG_FILE"
    fi

    # Report results
    if [ $actual_exit_code -eq "$expected_exit_code" ]; then
        echo -e "  ${GREEN}✓ PASSED${NC}"
        echo "  $message"
        return 0
    else
        echo -e "  ${RED}✗ FAILED${NC}"
        echo "  Expected exit code $expected_exit_code, got $actual_exit_code"
        echo "  Test message: $message"
        return 1
    fi
}

# Counter for tests
TESTS_TOTAL=0
TESTS_PASSED=0

# Run test and update counters
run_test_with_counter() {
    # show force flag
    echo "Running test: $1 with force flag: $5"

    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    if run_test "$@"; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    fi
}

# Test Cases
run_test_with_counter "Valid function docstring" '
/// This is a test function
/// It does something
fn test_function() {
    println!("Hello, world!");
}
' 0 "Should pass with valid docstring" false

run_test_with_counter "Valid nested impl" '
/// A test struct
pub struct TestStruct {
    /// A field
    field: String,
}

/// Implementation block
impl TestStruct {
    /// Creates a new instance
    pub fn new(field: String) -> Self {
        Self { field }
    }
}
' 0 "Should pass with nested implementations" false

run_test_with_counter "Force commit bypass" '
fn test_function() {
    println!("This function is missing its docstring!");
}
' 0 "Should pass with force flag" true

run_test_with_counter "Valid function docstring" '
/// This is a test function
/// It does something
fn test_function() {
    println!("Hello, world!");
}
' 0 "Should pass with valid docstring" false

run_test_with_counter "Valid public function requires docs" '
/// Public function documentation
pub fn public_function() {
    println!("Hello, world!");
}
' 0 "Should pass with documented public function" false

run_test_with_counter "Public function missing docs" '
pub fn public_function() {
    println!("Hello, world!");
}
' 1 "Should fail when public function is missing docs" false

run_test_with_counter "Private function without docstring" '
fn test_function() {
    println!("This function is missing its docstring!");
}
' 0 "Should pass - private functions don't require docstrings" false

run_test_with_counter "Public function without docstring" '
pub fn test_function() {
    println!("This function is missing its docstring!");
}
' 1 "Should fail - public functions require docstrings" false

run_test_with_counter "Valid nested impl" '
/// A test struct
pub struct TestStruct {
    /// A field
    field: String,
}

/// Implementation block
impl TestStruct {
    /// Creates a new instance
    pub fn new(field: String) -> Self {
        Self { field }
    }
}
' 0 "Should pass with nested implementations" false

run_test_with_counter "Force commit bypass" '
pub fn undocumented_function() {
    println!("This public function is missing docs!");
}
' 0 "Should pass with force flag" true

# Add test case from your actual codebase
run_test_with_counter "Camera module test" '
/// Camera module that handles the viewport and following the player
pub struct Camera {
    /// Position of the camera in world space
    pub position: Vec2,
    /// Size of the viewport
    pub viewport_size: Vec2,
}

impl Camera {
    /// Creates a new camera with the given viewport size
    pub fn new(viewport_size: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            viewport_size,
        }
    }
}
' 0 "Should pass with actual codebase structure" false

# Print summary
echo -e "\n${YELLOW}Test Summary:${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Total:  $TESTS_TOTAL"

# Cleanup
echo -e "\n${YELLOW}Cleaning up...${NC}"
cd "$PROJECT_ROOT"
rm -rf "$TEST_DIR"

if [ $TESTS_PASSED -eq $TESTS_TOTAL ]; then
    echo -e "${GREEN}All tests passed successfully!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed${NC}"
    exit 1
fi