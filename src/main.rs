use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

mod file_utils;

fn main() -> io::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ccwc <flag> [filename]");
        std::process::exit(1);
    }

    // Parse the flag
    let flag = &args[1];
    let input: Box<dyn Read> = if args.len() == 3 {
        let filename = &args[2];
        let file = File::open(filename)?;
        Box::new(BufReader::new(file)) as Box<dyn Read>
    } else {
        // No filename provided, read from standard input
        Box::new(io::stdin()) as Box<dyn Read>
    };

    // Handle different flags (-c for byte count, -l for line count)
    match flag.as_str() {
        "-c" => {
            let byte_count = file_utils::count_bytes(input)?;
            println!("  {}", byte_count);
        }
        "-l" => {
            let line_count = file_utils::count_lines(input)?;
            println!("  {}", line_count);
        }
        "-w" => {
            let word_count = file_utils::count_words(input)?;
            println!("  {}", word_count);
        }
        "-m" => {
            let char_count = file_utils::count_chars(input)?;
            println!("  {}", char_count);
        }
        "g" => {
            let mut contents = String::new();
            BufReader::new(input).read_to_string(&mut contents)?;
            let byte_count = file_utils::count_bytes_1(&contents)?;
            let line_count = file_utils::count_lines_1(&contents)?;
            let word_count = file_utils::count_words_1(&contents)?;
            println!("  {} {} {}", line_count, word_count, byte_count);
        }
        _ => {
            eprintln!(
                "Unknown flag: {}. Use -c for byte count, -l for line count, -w for word count, -m for character count, or g for all.",
                flag
            );
            std::process::exit(1);
        }
    }

    Ok(())
}

