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

pub fn count_words(file: File) -> io::Result<usize> {
    let reader = BufReader::new(file);
    let mut word_count = 0;

    for line in reader.lines() {
        let line = line?;
        // Split the line by whitespace and count words
        word_count += line.split_whitespace().count();
    }

    Ok(word_count)
}

pub fn count_chars(mut file: File) -> io::Result<usize> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents.chars().count())
}

/// Count the number of lines in the content.
pub fn count_lines_1(content: &str) -> io::Result<usize> {
    Ok(content.lines().count())
}

/// Count the number of words in the content.
pub fn count_words_1(content: &str) -> io::Result<usize> {
    Ok(content.split_whitespace().count())
}

/// Count the number of bytes in the content.
pub fn count_bytes_1(content: &str) -> io::Result<usize> {
    Ok(content.as_bytes().len())
}
