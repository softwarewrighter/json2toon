use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "json2toon")]
#[command(author = "Michael A Wright")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Convert JSON to TOON (Token-Oriented-Object-Notation) format")]
#[command(
    long_about = "json2toon - Convert JSON to TOON format\n\n\
    TOON (Token-Oriented-Object-Notation) is a flattened key-value format that represents\n\
    structured data with emphasis on token-level organization. Nested objects use dot notation,\n\
    and arrays use indexed notation.\n\n\
    Examples:\n  \
    json2toon input.json              # Convert input.json to input.toon\n  \
    json2toon input.json -o out.toon  # Convert with custom output\n  \
    json2toon -n input.json           # Dry run (preview)\n  \
    json2toon -v input.json           # Verbose output\n\n\
    Exit Codes:\n  \
    0 - Success\n  \
    1 - General error (I/O, conversion failure)\n  \
    2 - Usage error (invalid arguments)"
)]
#[command(help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
pub struct Args {
    /// Input JSON file to convert
    #[arg(value_name = "FILE")]
    pub input: PathBuf,

    /// Output TOON file (defaults to input with .toon extension)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Show version information
    #[arg(short = 'V', long)]
    pub version: bool,

    /// Dry run - show what would be done without modifying files
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// Verbose output - show detailed progress
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn get_output_path(&self) -> PathBuf {
        if let Some(ref output) = self.output {
            output.clone()
        } else {
            self.input.with_extension("toon")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_output_path() {
        let args = Args {
            input: PathBuf::from("test.json"),
            output: None,
            version: false,
            dry_run: false,
            verbose: false,
        };
        assert_eq!(args.get_output_path(), PathBuf::from("test.toon"));
    }

    #[test]
    fn test_custom_output_path() {
        let args = Args {
            input: PathBuf::from("test.json"),
            output: Some(PathBuf::from("custom.toon")),
            version: false,
            dry_run: false,
            verbose: false,
        };
        assert_eq!(args.get_output_path(), PathBuf::from("custom.toon"));
    }
}
