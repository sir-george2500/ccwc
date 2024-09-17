use std::io::{self, BufRead, Read};

/// Count the number of bytes from a `Read` trait object.
pub fn count_bytes<R: Read>(mut reader: R) -> io::Result<usize> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer.len())
}

/// Count the number of lines from a `Read` trait object.
pub fn count_lines<R: Read>(reader: R) -> io::Result<usize> {
    let reader = io::BufReader::new(reader);
    let line_count = reader.lines().count();
    Ok(line_count)
}

/// Count the number of words from a `Read` trait object.
pub fn count_words<R: Read>(reader: R) -> io::Result<usize> {
    let reader = io::BufReader::new(reader);
    let mut word_count = 0;

    for line in reader.lines() {
        let line = line?;
        word_count += line.split_whitespace().count();
    }

    Ok(word_count)
}

/// Count the number of characters from a `Read` trait object.
pub fn count_chars<R: Read>(mut reader: R) -> io::Result<usize> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents.chars().count())
}

/// Count the number of lines in the content string.
pub fn count_lines_1(content: &str) -> io::Result<usize> {
    Ok(content.lines().count())
}

/// Count the number of words in the content string.
pub fn count_words_1(content: &str) -> io::Result<usize> {
    Ok(content.split_whitespace().count())
}

/// Count the number of bytes in the content string.
pub fn count_bytes_1(content: &str) -> io::Result<usize> {
    Ok(content.as_bytes().len())
}
