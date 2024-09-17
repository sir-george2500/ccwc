# `ccwc` - Character, Word, and Byte Count Tool

![ccwc Screenshot](https://github.com/sir-george2500/custome_images/blob/main/images/wccscreensho.png)

## Description

`ccwc` is a command-line utility designed to count characters, words, lines, and bytes in text files or from standard input. It provides a flexible way to analyze text data, with support for various options to customize the type of counting performed.

## Author

- **Name**: Sir-George S Mulbah

## Features

### 1. **Character Count (`-m` option)**

   - **Description**: Counts the total number of characters in the given file or standard input. This includes all visible characters and whitespace.
   - **Usage**: `ccwc -m <filename>` or `cat <filename> | ccwc -m`
   - **Example**:
     ```bash
     $ ccwc -m test.txt
     342190 test.txt
     ```
     ```bash
     $ cat test.txt | ccwc -m
     342190
     ```

### 2. **Byte Count (`-c` option)**

   - **Description**: Counts the total number of bytes in the given file or standard input. This is useful for understanding the size of the file in bytes.
   - **Usage**: `ccwc -c <filename>` or `cat <filename> | ccwc -c`
   - **Example**:
     ```bash
     $ ccwc -c test.txt
     342190 test.txt
     ```
     ```bash
     $ cat test.txt | ccwc -c
     342190
     ```

### 3. **Line Count (`-l` option)**

   - **Description**: Counts the number of lines in the given file or standard input. This is useful for understanding the structure of the text in terms of line breaks.
   - **Usage**: `ccwc -l <filename>` or `cat <filename> | ccwc -l`
   - **Example**:
     ```bash
     $ ccwc -l test.txt
     714 test.txt
     ```
     ```bash
     $ cat test.txt | ccwc -l
     714
     ```

### 4. **Word Count (`-w` option)**

   - **Description**: Counts the total number of words in the given file or standard input. This helps in understanding the content length in terms of word count.
   - **Usage**: `ccwc -w <filename>` or `cat <filename> | ccwc -w`
   - **Example**:
     ```bash
     $ ccwc -w test.txt
     58164 test.txt
     ```
     ```bash
     $ cat test.txt | ccwc -w
     58164
     ```

### 5. **Comprehensive Count (`<no flag>` or `-g` option)**

   - **Description**: Provides a comprehensive count of lines, words, and bytes. When no flag is provided, it defaults to this mode, offering a summary of all three metrics.
   - **Usage**: `ccwc <filename>` or `cat <filename> | ccwc`
   - **Example**:
     ```bash
     $ ccwc test.txt
     714 58164 342190 test.txt
     ```
     ```bash
     $ cat test.txt | ccwc
     714 58164 342190
     ```

### 6. **Standard Input Support**

   - **Description**: `ccwc` can process data piped from standard input, allowing it to count characters, words, lines, and bytes from data streams.
   - **Usage**: `cat <filename> | ccwc <option>` or `echo "text" | ccwc <option>`
   - **Example**:
     ```bash
     $ echo "Hello world" | ccwc -w
     2
     ```

## Usage

To use `ccwc`, you need to specify at least one of the following options:

- `-c` for byte count
- `-l` for line count
- `-w` for word count
- `-m` for character count

If no option is provided, the tool defaults to comprehensive counting, showing the number of lines, words, and bytes.

### Examples

1. **Byte Count**
   ```bash
   $ ccwc -c test.txt
   342190 test.txt

