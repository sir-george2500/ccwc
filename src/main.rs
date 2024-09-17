use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

mod file_utils;

fn main() -> io::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a flag and filename were provided
    if args.len() < 3 {
        eprintln!("Usage: ccwc <flag> <filename>");
        std::process::exit(1);
    }

    // Parse the flag and filename
    let flag = &args[1];
    let filename = &args[2];

    // Open the file
    let file = File::open(filename)?;

    // Handle different flags (-c for byte count, -l for line count)
    match flag.as_str() {
        "-c" => {
            let byte_count = file_utils::count_bytes(file)?;
            println!("  {} {}", byte_count, filename);
        }
        "-l" => {
            let line_count = file_utils::count_lines(file)?;
            println!("  {} {}", line_count, filename);
        }
        "-w" => {
            let word_count = file_utils::count_words(file)?;
            println!("  {} {}", word_count, filename);
        }
        "-m" => {
            let char_count = file_utils::count_chars(file)?;
            println!("  {} {}", char_count, filename);
        }
        "g" => {
            let mut contents = String::new();
            BufReader::new(&file).read_to_string(&mut contents)?;
            let byte_count = file_utils::count_bytes_1(&contents)?;
            let line_count = file_utils::count_lines_1(&contents)?;
            let word_count = file_utils::count_words_1(&contents)?;
            println!(
                "  {} {} {} {}",
                line_count, word_count, byte_count, filename
            );
        }
        _ => {
            eprintln!(
                "Unknown flag: {}. Use -c for byte count or -l for line count.",
                flag
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
