# json2toon Implementation Plan

## Phase 1: Project Setup and Infrastructure

### Task 1.1: Project Configuration
- [x] Initialize Rust project with Cargo
- [ ] Update Cargo.toml with metadata and dependencies
- [ ] Add LICENSE file (MIT)
- [ ] Create .gitignore for Rust projects
- [ ] Set edition to 2024

### Task 1.2: Build System
- [ ] Create build.rs for compile-time metadata
- [ ] Add chrono dependency for timestamps
- [ ] Add hostname dependency for build host
- [ ] Test build script outputs environment variables

### Task 1.3: Documentation
- [ ] Create docs directory
- [ ] Write architecture.md
- [ ] Write prd.md
- [ ] Write design.md
- [ ] Write plan.md (this file)
- [ ] Write status.md

## Phase 2: Core Module Structure

### Task 2.1: CLI Module (`src/cli.rs`)
- [ ] Add clap dependency (v4 with derive feature)
- [ ] Define Args struct with all flags
- [ ] Implement argument parsing
- [ ] Add custom help text generation
- [ ] Differentiate between -h and --help

### Task 2.2: Version Module (`src/version.rs`)
- [ ] Create version display function
- [ ] Format version information output
- [ ] Include build metadata from environment variables
- [ ] Test version output format

### Task 2.3: Configuration Module (`src/config.rs`)
- [ ] Define Config struct
- [ ] Implement Config::from_args()
- [ ] Handle default output path generation
- [ ] Validate paths and options

## Phase 3: TOON Format Implementation

### Task 3.1: TOON Writer (`src/toon.rs`)
- [ ] Define ToonWriter struct
- [ ] Implement write methods for each JSON type
- [ ] Add string escaping logic
- [ ] Implement path flattening for nested objects
- [ ] Handle array indexing

### Task 3.2: TOON Tests
- [ ] Unit tests for string escaping
- [ ] Tests for nested object flattening
- [ ] Tests for array handling
- [ ] Tests for special characters
- [ ] Edge case tests

## Phase 4: JSON to TOON Converter

### Task 4.1: Converter Module (`src/converter.rs`)
- [ ] Add serde_json dependency
- [ ] Create Converter struct
- [ ] Implement JSON parsing
- [ ] Implement recursive TOON conversion
- [ ] Handle all JSON value types

### Task 4.2: Type Mapping
- [ ] String → TOON string
- [ ] Number → TOON number
- [ ] Boolean → TOON boolean
- [ ] Null → TOON null
- [ ] Object → flattened TOON
- [ ] Array → indexed TOON

### Task 4.3: Converter Tests
- [ ] Test simple types conversion
- [ ] Test nested objects conversion
- [ ] Test arrays conversion
- [ ] Test mixed types
- [ ] Test edge cases (empty objects/arrays, null values)

## Phase 5: File I/O and Main Logic

### Task 5.1: File Operations (`src/io.rs`)
- [ ] Implement file reading with error handling
- [ ] Implement atomic file writing (temp + rename)
- [ ] Add file size utilities
- [ ] Handle path operations

### Task 5.2: Main Application Logic (`src/main.rs`)
- [ ] Implement main() function
- [ ] Wire up CLI argument parsing
- [ ] Handle version flag
- [ ] Implement conversion pipeline
- [ ] Add error handling and reporting

### Task 5.3: Error Handling (`src/error.rs`)
- [ ] Add anyhow dependency
- [ ] Define error types
- [ ] Implement user-friendly error messages
- [ ] Add error context

## Phase 6: Special Modes

### Task 6.1: Dry-Run Mode
- [ ] Implement dry-run logic in main
- [ ] Create step-by-step preview
- [ ] Calculate estimated output size
- [ ] Format dry-run output
- [ ] Test dry-run doesn't modify files

### Task 6.2: Verbose Mode
- [ ] Add logging infrastructure
- [ ] Implement progress reporting
- [ ] Add timing information
- [ ] Log major operations
- [ ] Test verbose output format

### Task 6.3: Mode Combination Tests
- [ ] Test --dry-run --verbose
- [ ] Test error handling in each mode
- [ ] Validate output format consistency

## Phase 7: Testing and Validation

### Task 7.1: Unit Tests
- [ ] TOON writer tests (>90% coverage)
- [ ] Converter tests (>90% coverage)
- [ ] CLI parsing tests
- [ ] Helper function tests

### Task 7.2: Integration Tests
- [ ] Create test fixtures (JSON files)
- [ ] End-to-end conversion tests
- [ ] CLI integration tests
- [ ] Error scenario tests

### Task 7.3: Test Fixtures
- [ ] simple.json - basic types
- [ ] nested.json - nested objects
- [ ] arrays.json - array handling
- [ ] unicode.json - special characters
- [ ] edge_cases.json - empty, null, large numbers

## Phase 8: Code Quality and Modularity

### Task 8.1: Fix sw-checklist Failures
- [ ] Build release binary with `cargo build --release`
- [ ] Refactor `convert_value` function in converter.rs (currently 51 lines, max 50)
  - Break into smaller helper functions
  - Target: <25 lines per function
- [ ] Refactor `estimate_value_size` function in converter.rs (currently 41 lines)
  - Break into smaller helper functions
  - Target: <25 lines per function
- [ ] Refactor converter.rs module (currently 15 functions, max 7)
  - Consider splitting into multiple modules
  - Group related functions logically
  - Target: ≤4 functions per module
- [ ] Refactor toon.rs module (currently 16 functions, max 7)
  - Consider splitting into multiple modules (e.g., writer, formatter, escaping)
  - Group related functions logically
  - Target: ≤4 functions per module
- [ ] Refactor `main` function in main.rs (currently 98 lines, max 50)
  - Extract logic into separate functions
  - Move conversion pipeline to dedicated function
  - Move error handling to helper functions
  - Target: <25 lines per function
- [ ] Address crate module count (currently 5 modules, warning at >4)
  - Evaluate if modules can be consolidated
  - Or accept warning if modularity is beneficial

### Task 8.2: Code Quality
- [ ] Run clippy and fix warnings
- [ ] Format code with rustfmt
- [ ] Add documentation comments
- [ ] Review and refactor
- [ ] Re-run sw-checklist to verify all checks pass

### Task 8.3: User Documentation
- [ ] Write README.md
- [ ] Add usage examples
- [ ] Document TOON format
- [ ] Add troubleshooting section

### Task 8.4: Final Validation
- [ ] Test on Linux
- [ ] Test on macOS
- [ ] Test on Windows
- [ ] Validate all requirements from PRD
- [ ] Update status.md

## Phase 9: Release Preparation

### Task 9.1: Release Checklist
- [ ] Verify version is 0.1.0
- [ ] Check all tests pass
- [ ] Validate build on all platforms
- [ ] Generate release artifacts
- [ ] Tag git repository

### Task 9.2: GitHub Repository
- [ ] Push to GitHub
- [ ] Create GitHub release
- [ ] Add release notes
- [ ] Upload binary artifacts

## Dependencies Summary

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"

[build-dependencies]
chrono = "0.4"
hostname = "0.3"
```

## Timeline Estimate

- Phase 1: Setup (1 session)
- Phase 2: Core modules (1 session)
- Phase 3: TOON format (1 session)
- Phase 4: Converter (1 session)
- Phase 5: Main logic (1 session)
- Phase 6: Special modes (1 session)
- Phase 7: Testing (1 session)
- Phase 8: Polish (1 session)
- Phase 9: Release (1 session)

**Note**: Timeline assumes focused development sessions. Actual time may vary.

## Risk Mitigation

### Risk 1: TOON Format Ambiguity
- **Mitigation**: Document format clearly, create reference examples
- **Status**: Addressed in design.md

### Risk 2: Performance with Large Files
- **Mitigation**: Implement streaming if needed, benchmark with large files
- **Status**: To be monitored during implementation

### Risk 3: Cross-Platform Compatibility
- **Mitigation**: Test on all platforms, use platform-agnostic APIs
- **Status**: To be validated in Phase 8

## Success Criteria

- ✅ All PRD requirements implemented
- ✅ >80% test coverage
- ✅ All tests passing
- ✅ Clean clippy run
- ✅ Documentation complete
- ✅ Cross-platform validation
- ✅ Version 0.1.0 released
