pub fn print_version() {
    println!("json2toon {}", env!("CARGO_PKG_VERSION"));
    println!("Copyright (c) 2025 Michael A Wright");
    println!("License: MIT");
    println!(
        "Repository: https://github.com/softwarewrighter/json2toon"
    );
    println!();
    println!("Build Information:");
    println!("  Host: {}", env!("BUILD_HOST"));
    println!("  Commit: {}", env!("GIT_COMMIT_SHA"));
    println!("  Date: {}", env!("BUILD_TIMESTAMP"));
}
