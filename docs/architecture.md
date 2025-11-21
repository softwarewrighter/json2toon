# json2toon Architecture

## Overview

json2toon is a command-line tool written in Rust that converts JSON files to TOON (Token-Oriented-Object-Notation) format.

## System Architecture

```
┌─────────────────┐
│   CLI Entry     │  (main.rs)
│   (clap-based)  │
└────────┬────────┘
         │
         ├─── Version Info ──── build.rs (compile-time metadata)
         │
         ├─── Argument Parsing
         │    ├── -h/--help (user help)
         │    ├── --help (extended help for AI agents)
         │    ├── -V/--version
         │    ├── -n/--dry-run
         │    ├── -v/--verbose
         │    └── <input.json>
         │
         ├─── Input Processing
         │    └── JSON Parser (serde_json)
         │
         ├─── Core Converter
         │    ├── JSON → TOON Transformer
         │    └── TOON Serializer
         │
         └─── Output
              ├── Dry-run Mode (show steps)
              ├── Verbose Mode (show progress)
              └── Write TOON file
```

## Component Breakdown

### 1. CLI Layer (`main.rs`, `cli.rs`)
- Argument parsing using `clap` v4
- Command routing
- Error handling and user feedback

### 2. Version Module (`version.rs`)
- Compile-time metadata injection via `build.rs`
- Version, copyright, license display
- Build information (host, commit SHA, timestamp)

### 3. Converter Core (`converter.rs`)
- JSON parsing using `serde_json`
- JSON to TOON transformation logic
- Handles nested structures, arrays, and primitives

### 4. TOON Format (`toon.rs`)
- TOON data structure representation
- TOON serialization
- Format specification compliance

### 5. Modes
- **Dry-run Mode**: Preview conversion steps without writing files
- **Verbose Mode**: Real-time progress logging during conversion

## Data Flow

```
Input JSON File
      ↓
Parse JSON (serde_json)
      ↓
Convert to TOON AST
      ↓
Serialize TOON
      ↓
Output TOON File
```

## Build System

- **Cargo**: Standard Rust build system
- **build.rs**: Captures build-time metadata:
  - Git commit SHA
  - Build timestamp (ISO 8601)
  - Build hostname
  - Injects as environment variables for runtime access

## Dependencies

- `clap` - CLI argument parsing with derive macros
- `serde` - Serialization framework
- `serde_json` - JSON parsing
- `chrono` - Timestamp handling (build-time)
- `anyhow` - Error handling

## Error Handling Strategy

- Use `anyhow::Result` for propagating errors
- Provide user-friendly error messages
- Exit codes:
  - 0: Success
  - 1: General error
  - 2: Usage error

## Testing Strategy

- Unit tests for converter logic
- Integration tests for CLI interface
- Example JSON files for validation
- TOON format validation tests
