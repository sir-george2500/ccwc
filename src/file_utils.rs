use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

/// Count the number of bytes in a file.
pub fn count_bytes(mut file: File) -> io::Result<usize> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer.len())
}

/// Count the number of lines in a file.
pub fn count_lines(file: File) -> io::Result<usize> {
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count)
}
