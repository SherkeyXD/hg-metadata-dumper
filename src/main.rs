mod extractor;
mod hgxxtea;
mod metadata;
mod utils;

use extractor::Extractor;
use metadata::GlobalMetadata;
use std::env;
use std::fs::File;
use std::io::Write;
use utils::{fmt_bytes_hex, fmt_size};

struct Args {
    input: String,
    output: String,
    verbose: bool,
    decrypt_strings: bool,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        if args.iter().any(|arg| arg == "--help" || arg == "-h") {
            println!("Usage: {} [OPTIONS] [INPUT] [OUTPUT]", args[0]);
            println!("\nArguments:");
            println!("  [INPUT]     Input PE file path (default: GameAssembly.dll)");
            println!("  [OUTPUT]    Output decrypted file path (default: global-metadata.dat)");
            println!("\nOptions:");
            println!("  -d, --decrypt-strings  Decrypt string literals");
            println!("  -v, --verbose          Show detailed metadata info");
            println!("  -h, --help             Show this help message");
            std::process::exit(0);
        }

        let verbose = args.iter().any(|arg| arg == "--verbose" || arg == "-v");
        let decrypt_strings = args.iter().any(|arg| arg == "--decrypt-strings" || arg == "-d");
        let positional: Vec<&String> = args
            .iter()
            .skip(1)
            .filter(|arg| !arg.starts_with("--") && *arg != "-v" && *arg != "-h" && *arg != "-d")
            .collect();

        let input = positional
            .first()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "GameAssembly.dll".to_string());
        let output = positional
            .get(1)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "global-metadata.dat".to_string());

        Args { input, output, verbose, decrypt_strings }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut extractor = Extractor::new(&args.input);
    if let Err(e) = extractor.process() {
        eprintln!("✗ Extraction failed: {}", e);
        std::process::exit(1);
    }
    let data = extractor.get_valid_data();
    println!("✓ Extracted encrypted data from {}", args.input);
    println!();

    let key = {
        let mut k = vec![0u8; 16];
        k[0] = b'E';
        k[1] = b'8';
        k[2] = b'F';
        k[3] = b'F';
        k
    };

    let start_time = std::time::Instant::now();
    let mut decrypted = match hgxxtea::decrypt(data, &key) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("✗ Decryption failed: {}", e);
            std::process::exit(1);
        }
    };
    let duration = start_time.elapsed();
    println!("✓ Decrypted in {:.3}s", duration.as_secs_f64());
    println!();

    match GlobalMetadata::parse(&decrypted) {
        Ok(metadata) => {
            if metadata.is_valid() {
                println!("✓ Valid global-metadata.dat");
                if args.decrypt_strings {
                    if let Err(e) = metadata::decrypt_string_literals(&metadata, &mut decrypted) {
                        eprintln!("  Failed to decrypt string literals: {}", e);
                    }
                }

                println!("  Magic:   {}", fmt_bytes_hex(&metadata.magic_bytes()));
                println!("  Version: {}", metadata.header.version);
                if args.verbose {
                    println!("  String Literals:  {}", metadata.string_literals.len());
                    println!("  Images:           {}", metadata.images.len());
                    println!("  Assemblies:       {}", metadata.assemblies.len());
                    println!("  Type Definitions: {}", metadata.type_definitions.len());
                    println!("  Usage Lists:      {}", metadata.metadata_usage_lists.len());
                    println!("  Usage Pairs:      {}", metadata.metadata_usage_pairs.len());
                }
            } else {
                eprintln!("⚠ Header magic mismatch");
                eprintln!("  Expected: AF 1B B1 FA");
                eprintln!("  Got:      {}", fmt_bytes_hex(&metadata.magic_bytes()));
                eprintln!("  The decrypted data may not be a valid global-metadata.dat");
            }
        }
        Err(e) => {
            eprintln!("⚠ Failed to parse metadata: {}", e);
        }
    }
    println!();

    let mut out_file =
        File::create(&args.output).map_err(|e| format!("Failed to create output file: {}", e))?;
    out_file
        .write_all(&decrypted)
        .map_err(|e| format!("Failed to write output file: {}", e))?;

    println!("✓ Saved to {}", args.output);
    println!("  File size: {}", fmt_size(decrypted.len()));

    Ok(())
}
