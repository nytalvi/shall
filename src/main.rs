use clap::Parser;
use colored::*;
use md5::{Digest as Md5Digest, Md5};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "shall")]
#[command(about = "Calculate various hashes of a string or file")]
struct Args {
    /// Calculate SHA1 hash
    #[arg(long)]
    sha1: bool,

    /// Calculate SHA256 hash
    #[arg(long)]
    sha256: bool,

    /// Calculate SHA512 hash
    #[arg(long)]
    sha512: bool,

    /// Calculate MD5 hash
    #[arg(long)]
    md5: bool,

    /// Input file to hash
    #[arg(long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Get hashes for all files in a directory (non-recursive)
    #[arg(long, value_name = "DIR")]
    directory: Option<PathBuf>,

    /// Read input from stdin
    #[arg(long)]
    stdin: bool,

    /// Enable verbose output
    #[arg(long)]
    verbose: bool,

    /// The string to hash (ignored if --file or --stdin is specified)
    #[arg(required_unless_present_any = ["file", "stdin", "directory"])]
    input: Option<String>,
}

fn print_hash(name: &str, hash: &[u8]) {
    println!(
        "{} | {} | {}",
        name.blue().bold(),
        "-".cyan(),
        hex::encode(hash).cyan()
    );
}

fn print_file_hash(name: &str, file: &str, hash: &[u8]) {
    println!(
        "{} | {} | {}",
        name.blue().bold(),
        file.cyan(),
        hex::encode(hash).cyan()
    );
}

fn calculate_hashes(data: &[u8], args: &Args) {
    // If no specific algorithm is selected, show all
    let show_all = !args.sha1 && !args.sha256 && !args.sha512 && !args.md5;

    if args.verbose {
        println!("Input size: {} bytes", data.len());
    }

    // Calculate SHA1
    if show_all || args.sha1 {
        if args.verbose {
            print!("Calculating SHA1... ");
            io::stdout().flush().unwrap();
        }
        let mut sha1 = Sha1::new();
        sha1.update(data);
        print_hash("SHA1    ", &sha1.finalize());
    }

    // Calculate SHA256
    if show_all || args.sha256 {
        if args.verbose {
            print!("Calculating SHA256... ");
            io::stdout().flush().unwrap();
        }
        let mut sha256 = Sha256::new();
        sha256.update(data);
        print_hash("SHA256  ", &sha256.finalize());
    }

    // Calculate SHA512
    if show_all || args.sha512 {
        if args.verbose {
            print!("Calculating SHA512... ");
            io::stdout().flush().unwrap();
        }
        let mut sha512 = Sha512::new();
        sha512.update(data);
        print_hash("SHA512  ", &sha512.finalize());
    }

    // Calculate MD5
    if show_all || args.md5 {
        if args.verbose {
            print!("Calculating MD5... ");
            io::stdout().flush().unwrap();
        }
        let mut md5 = Md5::new();
        md5.update(data);
        print_hash("MD5     ", &md5.finalize());
    }
}

fn process_directory(dir: &PathBuf, args: &Args) -> io::Result<()> {
    // Ensure exactly one hash type is selected
    let hash_flags = [args.sha1, args.sha256, args.sha512, args.md5];
    let selected_count = hash_flags.iter().filter(|&&x| x).count();

    if selected_count != 1 {
        eprintln!("Error: When using --directory, exactly one hash type must be selected");
        std::process::exit(1);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        let data = fs::read(&path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        if args.sha1 {
            let mut sha1 = Sha1::new();
            sha1.update(&data);
            print_file_hash("SHA1", file_name, &sha1.finalize());
        } else if args.sha256 {
            let mut sha256 = Sha256::new();
            sha256.update(&data);
            print_file_hash("SHA256", file_name, &sha256.finalize());
        } else if args.sha512 {
            let mut sha512 = Sha512::new();
            sha512.update(&data);
            print_file_hash("SHA512", file_name, &sha512.finalize());
        } else if args.md5 {
            let mut md5 = Md5::new();
            md5.update(&data);
            print_file_hash("MD5", file_name, &md5.finalize());
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(dir) = args.directory.as_ref() {
        return process_directory(dir, &args);
    }

    let data: Vec<u8> = if let Some(file) = args.file.as_ref() {
        if args.verbose {
            println!("Reading from file: {}", file.display());
        }
        match fs::read(file) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("{}: {}", "Error reading file".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else if args.stdin {
        if args.verbose {
            println!("Reading from stdin...");
        }
        let mut buffer = Vec::new();
        match io::stdin().read_to_end(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => {
                eprintln!("{}: {}", "Error reading from stdin".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else {
        args.input.as_ref().unwrap().as_bytes().to_vec()
    };

    calculate_hashes(&data, &args);
    Ok(())
}
