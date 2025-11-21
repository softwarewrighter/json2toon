# json2toon Design Document

## Design Principles

1. **Simplicity First**: Clear, maintainable code over clever optimizations
2. **Unix Philosophy**: Do one thing well, compose with other tools
3. **User-Centric**: Prioritize clear error messages and intuitive UX
4. **Zero-Cost Abstractions**: Leverage Rust's performance without runtime overhead
5. **Fail Fast**: Validate input early, provide actionable feedback

## TOON Format Specification

### What is TOON?

TOON (Token-Oriented-Object-Notation) is a data format that represents structured data with emphasis on token-level organization.

### TOON Syntax (Assumed Specification)

Since TOON format isn't standardized, we define it here:

```toon
# Comments start with #
# Objects use key=value pairs
name="John Doe"
age=30
active=true

# Nested objects use dot notation
address.street="123 Main St"
address.city="Springfield"
address.zip=12345

# Arrays use indexed notation
hobbies.0="reading"
hobbies.1="cycling"
hobbies.2="coding"

# Null values
middle_name=null

# Type preservation
score=98.5
count=42
verified=true
optional=null
```

### JSON to TOON Mapping

| JSON Type | TOON Representation | Example |
|-----------|---------------------|---------|
| String    | `key="value"`       | `name="Alice"` |
| Number    | `key=value`         | `age=25` |
| Boolean   | `key=true/false`    | `active=true` |
| Null      | `key=null`          | `middle=null` |
| Object    | Flattened with dots | `user.name="Bob"` |
| Array     | Indexed with dots   | `items.0="first"` |

## Module Design

### Module: `main.rs`

**Responsibility**: Application entry point and orchestration

```rust
fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    if args.version {
        version::print_version();
        return Ok(());
    }

    let config = Config::from_args(args);
    run(config)
}
```

### Module: `cli.rs`

**Responsibility**: Command-line argument parsing

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "json2toon")]
#[command(about = "Convert JSON to TOON format", long_about = None)]
struct Args {
    /// Input JSON file
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output TOON file (defaults to input with .toon extension)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Show version information
    #[arg(short = 'V', long)]
    version: bool,

    /// Dry run - show what would be done
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}
```

### Module: `version.rs`

**Responsibility**: Version information display

```rust
pub fn print_version() {
    println!("json2toon {}", env!("CARGO_PKG_VERSION"));
    println!("Copyright (c) 2025 Michael A Wright");
    println!("License: MIT");
    println!("Repository: https://github.com/softwarewrighter/json2toon");
    println!();
    println!("Build Information:");
    println!("  Host: {}", env!("BUILD_HOST"));
    println!("  Commit: {}", env!("GIT_COMMIT_SHA"));
    println!("  Date: {}", env!("BUILD_TIMESTAMP"));
}
```

### Module: `converter.rs`

**Responsibility**: JSON to TOON conversion logic

```rust
pub struct Converter {
    verbose: bool,
}

impl Converter {
    pub fn convert(&self, json: &str) -> anyhow::Result<String> {
        let value: serde_json::Value = serde_json::from_str(json)?;
        let toon = self.to_toon(&value, "")?;
        Ok(toon)
    }

    fn to_toon(&self, value: &Value, prefix: &str) -> anyhow::Result<String> {
        // Recursive conversion logic
    }
}
```

### Module: `toon.rs`

**Responsibility**: TOON formatting and serialization

```rust
pub struct ToonWriter {
    buffer: String,
}

impl ToonWriter {
    pub fn write_string(&mut self, key: &str, value: &str) {
        self.buffer.push_str(&format!("{}=\"{}\"\n", key, escape(value)));
    }

    pub fn write_number(&mut self, key: &str, value: f64) {
        self.buffer.push_str(&format!("{}={}\n", key, value));
    }

    // ... other type handlers
}
```

### Build Script: `build.rs`

**Responsibility**: Capture build-time metadata

```rust
use std::process::Command;

fn main() {
    // Get git commit SHA
    let commit = Command::new("git")
        .args(&["rev-parse", "--short=8", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_COMMIT_SHA={}", commit.trim());

    // Get build timestamp
    let timestamp = chrono::Utc::now().to_rfc3339();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Get hostname
    let hostname = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_HOST={}", hostname);
}
```

## Error Handling Design

### Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Json2ToonError {
    #[error("Failed to read input file: {0}")]
    InputReadError(#[from] std::io::Error),

    #[error("Invalid JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("TOON conversion failed: {0}")]
    ConversionError(String),

    #[error("Failed to write output file: {0}")]
    OutputWriteError(String),
}
```

### User-Facing Messages

- Include context about what operation failed
- Suggest corrective actions
- Show relevant file paths
- For JSON errors, show line/column numbers

## CLI Interaction Examples

### Example 1: Basic Conversion

```bash
$ json2toon input.json
Converting input.json to input.toon... Done.
```

### Example 2: Version Information

```bash
$ json2toon --version
json2toon 0.1.0
Copyright (c) 2025 Michael A Wright
License: MIT
Repository: https://github.com/softwarewrighter/json2toon

Build Information:
  Host: macbook-pro.local
  Commit: a3f8c92d
  Date: 2025-11-21T10:30:45Z
```

### Example 3: Dry Run

```bash
$ json2toon -n input.json
[DRY RUN] Would perform the following steps:
  1. Read JSON from: input.json (1.2 KB)
  2. Parse JSON structure
  3. Convert to TOON format
  4. Write TOON to: input.toon (estimated 1.5 KB)

[DRY RUN] No files were modified.
```

### Example 4: Verbose Mode

```bash
$ json2toon -v large.json
[INFO] Reading input file: large.json
[INFO] File size: 5.3 MB
[INFO] Parsing JSON...
[INFO] JSON parsed successfully (2341 objects)
[INFO] Converting to TOON format...
[INFO] Processing object 1000/2341...
[INFO] Processing object 2000/2341...
[INFO] Conversion complete (2.8s)
[INFO] Writing output to: large.toon
[INFO] Output written: 6.1 MB
[SUCCESS] Conversion completed in 3.2s
```

## Testing Strategy

### Unit Tests

- JSON parsing edge cases
- TOON formatting for each type
- Path flattening algorithm
- String escaping

### Integration Tests

- CLI argument parsing
- End-to-end file conversion
- Error handling scenarios
- Dry-run validation

### Test Files

```
tests/
├── fixtures/
│   ├── simple.json
│   ├── nested.json
│   ├── arrays.json
│   ├── unicode.json
│   └── edge_cases.json
└── integration_tests.rs
```

## Performance Considerations

1. **Memory**: Stream large files rather than loading entirely into memory
2. **String Building**: Use `String::with_capacity()` for known sizes
3. **I/O**: Use buffered readers/writers
4. **Allocation**: Pre-allocate collections when size is known

## Security Considerations

1. **Path Traversal**: Validate output paths
2. **Resource Exhaustion**: Limit input file size (configurable)
3. **Malicious JSON**: Handle deeply nested structures gracefully
4. **File Permissions**: Respect umask, don't overwrite without confirmation
