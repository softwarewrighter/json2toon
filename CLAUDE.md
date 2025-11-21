# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

json2toon is a Rust CLI tool that converts JSON files to TOON (Token-Oriented-Object-Notation) format. TOON is a flattened key-value format where:
- Objects use dot notation (e.g., `user.name="John"`)
- Arrays use indexed notation (e.g., `items.0="first"`)
- Strings are quoted, numbers/booleans/null are unquoted

## Essential Commands

### Build and Run
```bash
# Debug build
cargo build

# Release build (required for sw-checklist validation)
cargo build --release

# Run the tool
cargo run -- input.json
cargo run -- input.json output.toon
cargo run -- -n input.json              # Dry-run mode
cargo run -- -v input.json              # Verbose mode
cargo run -- -V                         # Version info

# Run from release binary
./target/release/json2toon input.json
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in verbose mode
cargo test -- --nocapture --test-threads=1
```

### Code Quality
```bash
# Linting (strict mode for CI)
cargo clippy --all-targets --all-features -- -D warnings

# Formatting
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Generate documentation
cargo doc --open

# Validate against Software Wrighter standards
sw-checklist
sw-checklist --verbose
```

## Architecture

### Module Structure
```
src/
├── main.rs       - Entry point, CLI orchestration, file I/O (NEEDS REFACTORING: 98 lines)
├── cli.rs        - Clap-based argument parsing
├── version.rs    - Version display with build metadata
├── converter.rs  - JSON→TOON conversion logic (NEEDS REFACTORING: 51-line function, 15 functions total)
└── toon.rs       - TOON format writer (NEEDS REFACTORING: 16 functions total)
build.rs          - Compile-time metadata injection (git SHA, timestamp, hostname)
```

### Data Flow
1. **CLI Parsing** (`cli.rs`) - Parse arguments with clap
2. **Input Reading** (`main.rs`) - Read JSON file from disk
3. **JSON Parsing** (`converter.rs`) - Parse with serde_json into `Value`
4. **Recursive Conversion** (`converter.rs`) - Walk JSON tree, convert each value
5. **TOON Writing** (`toon.rs`) - Format as TOON key-value pairs
6. **Output** (`main.rs`) - Write TOON file or display dry-run results

### Key Design Patterns

**Build-time Metadata Injection**:
- `build.rs` captures git commit SHA, build timestamp, and hostname at compile time
- Injected as environment variables accessible via `env!()` macro in `version.rs`
- Powers the `-V/--version` flag requirements

**CLI Help Differentiation**:
- `-h`: Concise user help
- `--help`: Extended help with AI agent instructions (per Software Wrighter standards)

**Conversion Strategy**:
- Recursive descent through JSON tree
- Path accumulation using dot notation for objects
- Indexed notation for arrays (e.g., `items.0`, `items.1`)
- String escaping handles quotes and special characters

## Current Refactoring Needs

### Critical Issues (sw-checklist FAIL)
1. **main.rs:main()** - 98 lines (max 50)
   - Extract conversion pipeline logic
   - Extract error handling helpers
   - Target: <25 lines per function

2. **converter.rs:convert_value()** - 51 lines (max 50)
   - Split object/array handling into separate functions
   - Target: <25 lines per function

3. **converter.rs module** - 15 functions (max 7)
   - Consider splitting into submodules (e.g., `converter/core.rs`, `converter/helpers.rs`)
   - Group related functions logically

4. **toon.rs module** - 16 functions (max 7)
   - Consider splitting into submodules (e.g., `toon/writer.rs`, `toon/formatter.rs`, `toon/escaping.rs`)
   - Group related functions logically

### Warnings (sw-checklist WARN)
- **converter.rs:estimate_value_size()** - 41 lines (target <25)
- **Crate module count** - 5 modules (warning at >4, acceptable if justified)

### Modularity Philosophy
This project follows Software Wrighter LLC standards based on Miller's Law (7±2 rule):
- Functions: <25 lines ideal, 50 max
- Modules: ≤4 functions ideal, 7 max
- Crates: ≤4 modules ideal, 7 max

Small, focused units are easier to understand, test, and maintain.

## Testing Strategy

- **Unit tests**: Test individual functions in converter and TOON writer
- **Integration tests**: End-to-end conversion with test fixtures
- **Test coverage target**: >80%
- **All tests must be in Rust** (not Python or other languages)

## Development Workflow (AI Agent Process)

When a checkpoint is requested:
1. Run and fix failing tests (`cargo test`)
2. Fix linting issues (`cargo clippy --all-targets --all-features -- -D warnings`)
3. Format code (`cargo fmt`)
4. Update documentation
5. Manage git status (stage relevant changes, update .gitignore for irrelevant files)
6. Create logical commit and push

## Project Metadata

- **Version**: 0.1.0
- **License**: MIT
- **Copyright**: (c) 2025 Michael A Wright
- **Repository**: https://github.com/softwarewrighter/json2toon
- **Rust Edition**: 2024 (requires Rust 1.82.0+)

## Dependencies

```toml
clap = "4"          # CLI argument parsing with derive macros
serde = "1"         # Serialization framework
serde_json = "1"    # JSON parsing
anyhow = "1"        # Error handling
chrono = "0.4"      # Build-time timestamps (build-dependencies)
```

## Special Requirements

### Version Output Format
The `-V/--version` flag must display:
- Version number
- Copyright notice
- MIT license
- Repository URL
- Build hostname
- Git commit SHA (8+ characters)
- Build timestamp (ISO 8601)

### Help Output Requirements
- `-h`: User-focused, concise
- `--help`: Includes technical details, exit codes, TOON format spec, programmatic usage

### File Operations
- Use atomic writes (write to temp file, then rename)
- Handle Unicode and special characters correctly
- Validate input file existence before processing

## Out of Scope (v0.1.0)
- TOON→JSON reverse conversion
- Batch file conversion
- Watch mode
- Configuration files
- GUI interface

## Documentation
Comprehensive project documentation in `docs/`:
- `architecture.md` - System architecture and component breakdown
- `prd.md` - Product requirements and success criteria
- `design.md` - Detailed design decisions
- `plan.md` - Implementation plan and task breakdown (includes sw-checklist fixes)
- `process.md` - Development process guidelines
- `ai_agent_instructions.md` - AI agent workflow and quality standards
