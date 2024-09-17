use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a filename was provided
    if args.len() < 3 {
        eprintln!("Usage: ccwc -c <filename>");
        std::process::exit(1);
    }

    if args[1] != "-c" {
        eprintln!("Unknown flag: {}. Use -c for character count.", args[1]);
        std::process::exit(1);
    }

    let file_name = &args[2];

    let mut file = File::open(file_name)?;

    //read the file content content into a String
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    let buffer_count = buffer.len();

    // Output the result
    println!("  {} {}", buffer_count, file_name);

    Ok(())
}
