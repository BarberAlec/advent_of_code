use std::fs::File;
use std::io;
use std::io::prelude::*;


pub fn read_file_line_by_line(filename: &str) -> io::Result<Vec<String>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

// return first and last numeric characters of a alphanumeric string
pub fn last_and_first_numeric(line: &str) -> (i32, i32) {
    let mut first = 0;
    let mut last = 0;
    for c in line.chars() {
        if c.is_numeric() {
            if first == 0 {
                first = c.to_digit(10).unwrap() as i32;
            }
            last = c.to_digit(10).unwrap() as i32;
        }
    }
    (first, last)
}


pub fn trebuchet() -> i32 {
    let filename = "aoc_1/data.txt";
    let lines = match read_file_line_by_line(filename) {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut counter = 0;
    for line in lines {
        let (first, last) = last_and_first_numeric(&line);
        counter += first*10 + last;
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_line_by_line() {
        let filename = "aoc_1/test_data.txt";
        let lines = read_file_line_by_line(filename).unwrap();
        assert_eq!(lines, vec!["Hello", "World"]);
    }

    #[test]
    fn test_last_and_first_numeric() {
        let line = "5413sxmtrdjtcmdqpbpcprsix";
        let (first, last) = last_and_first_numeric(line);
        assert_eq!(first, 5);
        assert_eq!(last, 3);
    }

    #[test]
    fn test_last_and_first_numeric_zeros() {
        let line = "sxmtrdjtcmdqpbpcprsix";
        let (first, last) = last_and_first_numeric(line);
        assert_eq!(first, 0);
        assert_eq!(last, 0);
    }

    #[test]
    fn test_last_and_first_numeric_one_digit() {
        let line = "sxmtrdjtc7mdqpbpcprsix";
        let (first, last) = last_and_first_numeric(line);
        assert_eq!(first, 7);
        assert_eq!(last, 7);
    }
}
