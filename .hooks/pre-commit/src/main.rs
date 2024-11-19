use std::fmt::Display;
use std::process::{exit, Command};
use std::{env, fs};
use std::path::Path;

// Debug logging macro
const DEBUG: bool = false;
macro_rules! debug {
    ($($arg:tt)*) => {
        if DEBUG { eprintln!("[DEBUG] {}", format!($($arg)*));}
    }
}

#[derive(Debug, Clone)]
struct Block {
    start: usize,
    is_public: bool,
    kind: BlockKind,
    docstring: Option<String>,
    nested_blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
enum BlockKind {
    Function,
    Struct,
    Impl,
    Trait,
    Module,
    Enum,
    TypeAlias,
    Unknown,
}
fn should_skip_file(file: &str) -> bool {
    // Skip checking the pre-commit hook itself
    let hook_file = Path::new(file);
    if let Some(file_name) = hook_file.file_name() {
        if let Some(name) = file_name.to_str() {
            if name == "main.rs" {
                // Check if it's in a pre-commit directory
                if let Some(parent) = hook_file.parent() {
                    if let Some(parent_name) = parent.file_name() {
                        if let Some(dir_name) = parent_name.to_str() {
                            return dir_name == "src" &&
                                parent.parent().and_then(|p| p.file_name())
                                    .and_then(|n| n.to_str())
                                    .map_or(false, |n| n == "pre-commit");
                        }
                    }
                }
            }
        }
    }
    false
}


fn parse_blocks(content: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut stack = Vec::new();
    let mut current_docstring: Option<String> = None;
    let lines: Vec<&str> = content.lines().collect();
    let mut brace_count = 0;
    let mut in_enum = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Track docstrings
        if trimmed.starts_with("///") || trimmed.starts_with("/**") {
            let doc = if let Some(existing) = current_docstring {
                existing + "\n" + trimmed
            } else {
                trimmed.to_string()
            };
            current_docstring = Some(doc);
            continue;
        }

        // Reset docstring if we hit a blank line
        if trimmed.is_empty() {
            current_docstring = None;
            continue;
        }

        // Count braces
        brace_count += trimmed.matches('{').count() as i32;
        brace_count -= trimmed.matches('}').count() as i32;

        // Track if we're inside an enum
        if trimmed.starts_with("enum ") || trimmed.contains(" enum ") {
            in_enum = true;
        }

        // Detect block starts
        if let Some((kind, is_public)) = detect_block_kind(trimmed) {
            debug!(
                "Found block: {:?} (public: {}) at line {}",
                kind,
                is_public,
                i + 1
            );

            // Create new block
            let block = Block {
                start: i,
                is_public,
                kind: kind.clone(),
                docstring: current_docstring.take(),
                nested_blocks: Vec::new(),
            };

            // Handle nesting
            if stack.is_empty() {
                blocks.push(block);
            } else if let Some(parent) = blocks.last_mut() {
                parent.nested_blocks.push(block);
            }

            stack.push((i, kind, current_docstring.take(), is_public));
            continue;
        }

        // Handle enum variant structs
        if in_enum && trimmed.contains('{') {
            let block = Block {
                start: i,
                is_public: false, // Variant visibility follows enum
                kind: BlockKind::Unknown,
                docstring: current_docstring.take(),
                nested_blocks: Vec::new(),
            };

            if let Some(parent) = blocks.last_mut() {
                parent.nested_blocks.push(block);
            }
        }

        // Track block ends
        if brace_count <= 0 && !stack.is_empty() {
            if let Some((_, kind, _, _)) = stack.pop() {
                if kind == BlockKind::Enum {
                    in_enum = false;
                }
            }
            brace_count = 0;
        }
    }

    blocks
}

fn detect_block_kind(line: &str) -> Option<(BlockKind, bool)> {
    let line = line.trim_start();
    let is_public = line.starts_with("pub ") || line.contains(" pub ");

    let line_without_pub = line.replace("pub ", "");
    let trimmed = line_without_pub.trim();

    let kind = if trimmed.starts_with("fn ") || trimmed.contains(" fn ") {
        Some(BlockKind::Function)
    } else if trimmed.starts_with("struct ") || trimmed.contains(" struct ") {
        Some(BlockKind::Struct)
    } else if trimmed.starts_with("impl ") {
        Some(BlockKind::Impl)
    } else if trimmed.starts_with("trait ") || trimmed.contains(" trait ") {
        Some(BlockKind::Trait)
    } else if trimmed.starts_with("mod ") || trimmed.contains(" mod ") {
        Some(BlockKind::Module)
    } else if trimmed.starts_with("enum ") || trimmed.contains(" enum ") {
        Some(BlockKind::Enum)
    } else if trimmed.starts_with("type ") || trimmed.contains(" type ") {
        Some(BlockKind::TypeAlias)
    } else if line.contains('{') && !line.contains("=>") {
        // Check for enum variant struct by looking at context
        // If it's indented and contains a brace, it's likely a nested block
        let indent_level = line.chars().take_while(|c| c.is_whitespace()).count();
        if indent_level > 0 {
            Some(BlockKind::Unknown)
        } else {
            None
        }
    } else {
        None
    };

    kind.map(|k| (k, is_public))
}

fn check_blocks(blocks: &[Block]) -> Vec<String> {
    let mut violations = Vec::new();

    // Get the git diff only once
    let git_diff = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .map_err(|e| {
            debug!("Error getting git diff: {}", e);
            e
        })
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok());

    // Debug the git diff
    if let Some(diff) = &git_diff {
        debug!("Git diff contents:\n{}", diff);
    } else {
        debug!("No git diff available");
    }

    for block in blocks {
        match block.kind {
            BlockKind::Impl => {
                // Check impl blocks contents
                for nested in &block.nested_blocks {
                    // Check public methods in impl blocks
                    if nested.is_public && nested.docstring.is_none() {
                        violations.push(format!(
                            "Public {} in implementation block at line {} is missing documentation",
                            nested.kind.to_string().to_lowercase(),
                            nested.start + 1
                        ));
                    }
                }
                violations.extend(check_blocks(&block.nested_blocks));
            }
            _ => {
                // For public items, always require documentation
                if block.is_public && block.docstring.is_none() {
                    violations.push(format!(
                        "Public {} at line {} is missing documentation",
                        block.kind.to_string(),
                        block.start + 1
                    ));
                } else if !block.is_public && block.docstring.is_none() {
                    // For private items, check if docs were removed
                    if let Some(diff) = &git_diff {
                        if diff
                            .lines()
                            .filter(|line| line.starts_with('-'))
                            .any(|line| line.contains("///") || line.contains("/**"))
                        {
                            violations.push(format!(
                                "Private {} at line {} had documentation that was removed",
                                block.kind.to_string(),
                                block.start + 1
                            ));
                        }
                    }
                }

                // Recursively check nested blocks
                violations.extend(check_blocks(&block.nested_blocks));
            }
        }
    }

    debug!("Found {} violations", violations.len());
    if !violations.is_empty() {
        debug!("Violations: {:?}", violations);
    }

    violations
}
fn check_force_flag() -> bool {
    debug!("Checking for force flag");

    // Check environment variable
    if env::var("GIT_COMMIT_FORCE").unwrap_or_default() == "1" {
        debug!("Force flag found in environment variable");
        return true;
    }

    // Check commit message from args
    if let Some(commit_msg_file) = env::args().nth(1) {
        debug!("Checking commit message file: {}", commit_msg_file);
        match fs::read_to_string(&commit_msg_file) {
            Ok(content) => {
                let force_present = content.contains("[force]");
                debug!("Force flag in commit message: {}", force_present);
                return force_present;
            }
            Err(e) => debug!("Error reading commit message file: {}", e),
        }
    } else {
        debug!("No commit message file provided in args");
    }

    false
}

fn main() {
    debug!("Starting pre-commit hook");
    debug!("Args: {:?}", env::args().collect::<Vec<_>>());
    debug!("Current dir: {:?}", env::current_dir());
    debug!(
        "Env vars: GIT_COMMIT_FORCE={:?}",
        env::var("GIT_COMMIT_FORCE")
    );

    if check_force_flag() {
        debug!("Force flag detected - skipping checks");
        exit(0);
    }

    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .expect("Failed to execute git command");

    let staged_files = String::from_utf8(output.stdout).expect("Failed to read git output");

    debug!("Staged files: {}", staged_files);

    let mut needs_review = false;

    for file in staged_files.lines().filter(|f| f.ends_with(".rs")) {
        // Skip the pre-commit hook's own file
        if should_skip_file(file) {
            debug!("Skipping pre-commit hook file: {}", file);
            continue;
        }

        debug!("Checking file: {}", file);

        let current_content = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(e) => {
                debug!("Error reading file {}: {}", file, e);
                continue;
            }
        };

        let current_blocks = parse_blocks(&current_content);
        debug!("Found {} blocks in {}", current_blocks.len(), file);

        let violations = check_blocks(&current_blocks);

        debug!("Found {} violations", violations.len());
        debug!("Violations: {:?}", violations);

        if !violations.is_empty() {
            println!("⚠️  In file {}:", file);
            for violation in violations {
                println!("   {}", violation);
            }
            needs_review = true;
        }
    }

    if needs_review {
        println!("\nTo bypass these checks:");
        println!("1. Add [force] to commit message");
        println!("2. Set GIT_COMMIT_FORCE=1");
        exit(1);
    }

    debug!("All checks passed");
    exit(0);
}

impl Display for BlockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BlockKind::Function => "Function",
            BlockKind::Struct => "Struct",
            BlockKind::Impl => "Implementation",
            BlockKind::Trait => "Trait",
            BlockKind::Module => "Module",
            BlockKind::Enum => "Enum",
            BlockKind::TypeAlias => "Type alias",
            BlockKind::Unknown => "Block",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that the block kind is correctly detected
    #[test]
    fn test_detect_block_kind() {
        assert_eq!(
            detect_block_kind("fn test() {"),
            Some((BlockKind::Function, false))
        );
        assert_eq!(
            detect_block_kind("pub fn test() {"),
            Some((BlockKind::Function, true))
        );
        assert_eq!(
            detect_block_kind("struct Test {"),
            Some((BlockKind::Struct, false))
        );
        assert_eq!(
            detect_block_kind("pub struct Test {"),
            Some((BlockKind::Struct, true))
        );
        assert_eq!(
            detect_block_kind("enum Test {"),
            Some((BlockKind::Enum, false))
        );
        assert_eq!(
            detect_block_kind("pub enum Test {"),
            Some((BlockKind::Enum, true))
        );
        assert_eq!(
            detect_block_kind("impl Test {"),
            Some((BlockKind::Impl, false))
        );
        assert_eq!(
            detect_block_kind("pub impl Test {"),
            Some((BlockKind::Impl, true))
        );
        assert_eq!(
            detect_block_kind("mod test {"),
            Some((BlockKind::Module, false))
        );
        assert_eq!(
            detect_block_kind("pub mod test {"),
            Some((BlockKind::Module, true))
        );
        assert_eq!(
            detect_block_kind("pub type Test = i32;"),
            Some((BlockKind::TypeAlias, true))
        );
        assert_eq!(
            detect_block_kind("type Test = i32;"),
            Some((BlockKind::TypeAlias, false))
        );
        assert_eq!(
            detect_block_kind("pub trait Test {"),
            Some((BlockKind::Trait, true))
        );
        assert_eq!(
            detect_block_kind("trait Test {"),
            Some((BlockKind::Trait, false))
        );
    }

    #[test]
    fn test_parse_blocks() {
        let content = r#"
            /// Doc comment
            pub fn test() {
                println!("test");
            }
        "#;
        let blocks = parse_blocks(content);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].is_public);
        assert_eq!(blocks[0].kind, BlockKind::Function);
        assert!(blocks[0].docstring.is_some());
    }

    #[test]
    fn test_parse_enum_blocks() {
        let content = r#"
            pub enum Test {
                A,
                B,
            }
        "#;
        let blocks = parse_blocks(content);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].is_public);
        assert_eq!(blocks[0].kind, BlockKind::Enum);
        assert!(blocks[0].docstring.is_none());
    }

    #[test]
    fn test_parse_nested_blocks() {
        let content = r#"
            pub enum Test {
                A,
                B,
                C {
                    a: i32,
                    b: f32,
                },
            }
        "#;
        let blocks = parse_blocks(content);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].is_public);
        assert_eq!(blocks[0].kind, BlockKind::Enum);
        assert!(blocks[0].docstring.is_none());
        assert_eq!(blocks[0].nested_blocks.len(), 1);
    }

    #[test]
    fn test_private_function_no_docs() {
        let content = r#"
            fn private_function() {
                println!("Hello");
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            violations.is_empty(),
            "Private functions should not require docs"
        );
    }

    #[test]
    fn test_public_function_requires_docs() {
        let content = r#"
            pub fn public_function() {
                println!("Hello");
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            !violations.is_empty(),
            "Public functions should require docs"
        );
    }

    #[test]
    fn test_impl_block_public_method() {
        let content = r#"
            impl Test {
                pub fn public_method() {
                    println!("Hello");
                }
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            !violations.is_empty(),
            "Public methods in impl should require docs"
        );
    }

    #[test]
    fn test_impl_block_private_method() {
        let content = r#"
            impl Test {
                fn private_method() {
                    println!("Hello");
                }
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            violations.is_empty(),
            "Private methods should not require docs"
        );
    }

    #[test]
    fn test_private_function_no_docs_should_pass() {
        let content = r#"
            fn private_function() {
                println!("Hello");
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            violations.is_empty(),
            "Private functions should not require docs"
        );
    }

    #[test]
    fn test_private_function_docs_not_removed() {
        let content = r#"
            fn private_function() {
                // Regular comment
                println!("Hello");
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            violations.is_empty(),
            "Private function with no previous docs should pass"
        );
    }

    #[test]
    fn test_private_function_with_removed_docs() {
        // This test would need to be run in a git context
        let content = r#"
            fn private_function() {
                println!("Hello");
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            violations.is_empty(),
            "Private function should not flag removed docs without git history"
        );
    }

    #[test]
    fn test_public_function_with_docs() {
        let content = r#"
            /// This is a documented public function
            pub fn public_function() {
                println!("Hello");
            }
        "#;
        let blocks = parse_blocks(content);
        let violations = check_blocks(&blocks);
        assert!(
            violations.is_empty(),
            "Public function with docs should pass"
        );
    }
}
