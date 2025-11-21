mod cli;
mod converter;
mod toon;
mod version;

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::io::Write;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    // Handle version flag
    if args.version {
        version::print_version();
        return Ok(());
    }

    // Validate input file exists
    if !args.input.exists() {
        anyhow::bail!("Input file does not exist: {}", args.input.display());
    }

    if !args.input.is_file() {
        anyhow::bail!("Input path is not a file: {}", args.input.display());
    }

    let output_path = args.get_output_path();

    // Read input file
    if args.verbose {
        println!("[INFO] Reading input file: {}", args.input.display());
    }

    let json_content = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input.display()))?;

    if args.verbose {
        let size_kb = json_content.len() as f64 / 1024.0;
        println!("[INFO] File size: {:.1} KB", size_kb);
    }

    // Convert JSON to TOON
    let converter = converter::Converter::new(args.verbose);

    if args.dry_run {
        // Dry run mode
        println!("[DRY RUN] Would perform the following steps:");

        let size_kb = json_content.len() as f64 / 1024.0;
        println!("  1. Read JSON from: {} ({:.1} KB)", args.input.display(), size_kb);
        println!("  2. Parse JSON structure");

        // Try to estimate output size
        match converter.estimate_size(&json_content) {
            Ok(estimated_size) => {
                let est_kb = estimated_size as f64 / 1024.0;
                println!("  3. Convert to TOON format");
                println!("  4. Write TOON to: {} (estimated {:.1} KB)", output_path.display(), est_kb);
            }
            Err(e) => {
                println!("  3. [ERROR] Failed to parse JSON: {}", e);
                println!("\n[DRY RUN] No files were modified.");
                std::process::exit(1);
            }
        }

        println!("\n[DRY RUN] No files were modified.");
        return Ok(());
    }

    // Perform actual conversion
    let toon_content = converter.convert(&json_content)
        .context("Failed to convert JSON to TOON")?;

    // Write output file
    if args.verbose {
        println!("[INFO] Writing output to: {}", output_path.display());
    }

    // Write to a temporary file first, then rename (atomic operation)
    let temp_path = output_path.with_extension("toon.tmp");

    let mut file = fs::File::create(&temp_path)
        .with_context(|| format!("Failed to create output file: {}", temp_path.display()))?;

    file.write_all(toon_content.as_bytes())
        .with_context(|| format!("Failed to write to output file: {}", temp_path.display()))?;

    file.sync_all()
        .context("Failed to sync output file to disk")?;

    drop(file);

    fs::rename(&temp_path, &output_path)
        .with_context(|| format!("Failed to rename temporary file to: {}", output_path.display()))?;

    if args.verbose {
        let size_kb = toon_content.len() as f64 / 1024.0;
        println!("[INFO] Output written: {:.1} KB", size_kb);
        println!("[SUCCESS] Conversion completed");
    } else {
        println!("Converted {} to {}", args.input.display(), output_path.display());
    }

    Ok(())
}
