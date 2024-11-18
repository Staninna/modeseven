#!/bin/bash

# Enable error handling
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Project specific paths
PROJECT_ROOT="$(git rev-parse --show-toplevel)"
HOOK_PATH="$PROJECT_ROOT/.hooks/pre-commit/target/release/pre-commit"

# Build the hook
cd "$PROJECT_ROOT/.hooks/pre-commit"
cargo build --release

echo -e "${YELLOW}Starting advanced pre-commit hook tests...${NC}"

# Check if hook exists and is compiled
if [ ! -f "$HOOK_PATH" ]; then
    echo -e "${RED}Error: Pre-commit hook not found at $HOOK_PATH${NC}"
    exit 1
fi

# Create temporary test repository
TEST_DIR=$(mktemp -d)
echo "Created test directory: $TEST_DIR"

# Initialize git repo
cd "$TEST_DIR"
git init -b main > /dev/null 2>&1
git config --local user.name "Test User"
git config --local user.email "test@example.com"

# Install the hook
mkdir -p .git/hooks
cp "$HOOK_PATH" .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Function to attempt a commit
attempt_commit() {
    local message=$1
    local force=${2:-false}
    local expected_exit_code=$3

    COMMIT_MSG_FILE=$(mktemp)
    echo "$message" > "$COMMIT_MSG_FILE"

    if [ "$force" = true ]; then
        GIT_COMMIT_FORCE=1 .git/hooks/pre-commit "$COMMIT_MSG_FILE"
    else
        .git/hooks/pre-commit "$COMMIT_MSG_FILE"
    fi

    local actual_exit_code=$?
    rm "$COMMIT_MSG_FILE"

    return $actual_exit_code
}

# Function to create a file and attempt to commit it
test_commit() {
    local test_name=$1
    local content=$2
    local commit_msg=$3
    local force=${4:-false}
    local expected_exit_code=$5

    echo -e "\n${BLUE}Running test: ${NC}$test_name"
    echo "$content" > test.rs
    git add test.rs

    if attempt_commit "$commit_msg" "$force" "$expected_exit_code"; then
        if [ "$expected_exit_code" -eq 0 ]; then
            echo -e "  ${GREEN}✓ PASSED${NC} - Commit succeeded as expected"
            git commit -m "$commit_msg" > /dev/null 2>&1
            return 0
        else
            echo -e "  ${RED}✗ FAILED${NC} - Commit succeeded but should have failed"
            return 1
        fi
    else
        if [ "$expected_exit_code" -eq 1 ]; then
            echo -e "  ${GREEN}✓ PASSED${NC} - Commit failed as expected"
            return 0
        else
            echo -e "  ${RED}✗ FAILED${NC} - Commit failed but should have succeeded"
            return 1
        fi
    fi
}

# Test Cases

# Test 1: Complex nested structure with mixed documentation
test_commit "Complex nested structure" '
/// Outer module documentation
pub mod complex {
    /// Inner type alias
    pub type ComplexResult<T> = Result<T, String>;

    /// Configuration structure
    pub struct Config {
        /// Database URL
        pub db_url: String,
        internal_setting: u32,
    }

    impl Config {
        /// Creates a new configuration
        pub fn new(db_url: String) -> Self {
            Self {
                db_url,
                internal_setting: 42,
            }
        }

        fn private_helper() {
            // This should not require docs
        }
    }

    /// Error enum
    pub enum Error {
        /// Database connection failed
        DbError,
        /// Invalid configuration
        ConfigError {
            /// Error message
            message: String,
            code: i32,
        },
    }
}' "Initial complex structure" false 0

# Test 2: Removing documentation from public items
test_commit "Removing docs from public items" '
pub mod complex {
    pub type ComplexResult<T> = Result<T, String>;

    pub struct Config {
        pub db_url: String,
        internal_setting: u32,
    }

    impl Config {
        pub fn new(db_url: String) -> Self {
            Self {
                db_url,
                internal_setting: 42,
            }
        }

        fn private_helper() {
        }
    }

    pub enum Error {
        DbError,
        ConfigError {
            message: String,
            code: i32,
        },
    }
}' "Remove documentation" false 1

# Test 3: Complex trait implementation
test_commit "Complex trait implementation" '
/// Database interface trait
pub trait Database {
    /// Connection type
    type Connection;

    /// Opens a new database connection
    fn connect(&self) -> Self::Connection;

    /// Executes a query
    fn query(&self, conn: &Self::Connection, query: &str) -> Result<(), String>;
}

/// PostgreSQL implementation
pub struct PostgresDb {
    /// Connection string
    url: String,
}

/// Implementation for PostgreSQL
impl Database for PostgresDb {
    /// Connection type
    type Connection = String;

    /// Opens a PostgreSQL connection
    fn connect(&self) -> Self::Connection {
        self.url.clone()
    }

    /// Executes a PostgreSQL query
    fn query(&self, conn: &Self::Connection, query: &str) -> Result<(), String> {
        Ok(())
    }
}' "Add database trait" false 0

# Test 4: Multiple commits in sequence
echo -e "\n${YELLOW}Testing multiple commits in sequence...${NC}"

# First commit: Initial implementation
test_commit "Initial implementation" '
/// User management module
pub mod users {
    /// User structure
    pub struct User {
        /// User ID
        pub id: u64,
        /// Username
        pub username: String,
    }
}' "Initial user module" false 0

# Second commit: Add functionality
test_commit "Add functionality" '
/// User management module
pub mod users {
    /// User structure
    pub struct User {
        /// User ID
        pub id: u64,
        /// Username
        pub username: String,
    }

    impl User {
        /// Creates a new user
        pub fn new(username: String) -> Self {
            Self {
                id: 0,
                username,
            }
        }
    }
}' "Add user constructor" false 0

# Third commit: Add more functionality without docs (should fail)
test_commit "Missing docs" '
/// User management module
pub mod users {
    /// User structure
    pub struct User {
        /// User ID
        pub id: u64,
        /// Username
        pub username: String,
    }

    impl User {
        /// Creates a new user
        pub fn new(username: String) -> Self {
            Self {
                id: 0,
                username,
            }
        }

        pub fn validate(&self) -> bool {
            !self.username.is_empty()
        }
    }
}' "Add validation" false 1

# Test 5: Force commit with missing docs
test_commit "Force commit with missing docs" '
pub mod forced {
    pub struct UndocumentedStruct {
        pub field: String,
    }

    pub fn undocumented_function() {
        println!("No docs here!");
    }
}' "Force commit [force]" true 0

# Test 6: Complex generic implementations
test_commit "Complex generic implementations" '
/// Generic data structure
pub struct DataStructure<T, U> where T: Clone, U: Default {
    /// Primary data
    pub data: T,
    /// Secondary data
    pub metadata: U,
}

/// Implementation for string data
impl<U: Default> DataStructure<String, U> {
    /// Creates a new instance with string data
    pub fn new_string(data: &str) -> Self {
        Self {
            data: data.to_string(),
            metadata: U::default(),
        }
    }
}

/// General implementation
impl<T: Clone, U: Default> DataStructure<T, U> {
    /// Maps the data to a new type
    pub fn map<V>(&self, f: impl Fn(&T) -> V) -> DataStructure<V, U>
    where
        V: Clone,
    {
        DataStructure {
            data: f(&self.data),
            metadata: U::default(),
        }
    }
}' "Add generic structure" false 0

# Test 7: Module with complex visibility rules
test_commit "Complex visibility rules" '
/// Outer module
pub(crate) mod visibility {
    /// Inner module
    pub(super) mod inner {
        /// Helper function
        pub(in crate::visibility) fn helper() {}

        /// Public function
        pub fn public_function() {}
    }

    /// Re-exported type
    pub use inner::public_function;
}' "Add visibility test" false 0

# Test 8: Derive macros and attributes
test_commit "Derive macros and attributes" '
/// Custom error type
#[derive(Debug, Clone, PartialEq)]
pub enum CustomError {
    /// IO operation failed
    #[allow(dead_code)]
    IoError(String),

    /// Validation failed
    #[cfg(feature = "validation")]
    ValidationError {
        /// Error message
        message: String,
    },
}

/// Handler implementation
#[cfg(test)]
impl CustomError {
    /// Creates a new IO error
    pub fn io_error(msg: &str) -> Self {
        Self::IoError(msg.to_string())
    }
}' "Add custom error" false 0

# Cleanup
echo -e "\n${YELLOW}Cleaning up...${NC}"
cd "$PROJECT_ROOT"
rm -rf "$TEST_DIR"

echo -e "\n${GREEN}Advanced tests completed!${NC}"