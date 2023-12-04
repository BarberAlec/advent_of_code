use super::part_1::read_file_line_by_line;
use std::collections::HashMap;
use lazy_static::lazy_static;


lazy_static! {
    static ref MAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };
}

// get first and last numeric characters or spelling of a number from a string
fn last_and_first_numeric(line: &str) -> (i32, i32) {
    let mut first = 0;
    let mut last = 0;
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if first == 0 {
                first = c.to_digit(10).unwrap() as i32;
            }
            last = c.to_digit(10).unwrap() as i32;
        }
        else if c.is_alphabetic() {
            let mut word = String::new();
            word.push(c);
            for j in i+1..line.len() {
                let c = line.chars().nth(j).unwrap();
                if c.is_alphabetic() {
                    word.push(c);
                } else {
                    break;
                }
                if let Some(&num) = MAP.get(word.as_str()) {
                    if first == 0 {
                        first = num;
                    }
                    last = num;
                    break;
                }
            }
        }
    }
    (first, last)

}

pub fn trebuchet() -> i32 {
    let filename = "aoc_1/data.txt";
    let lines = match read_file_line_by_line(filename) {
        Ok(lines) => lines,
        Err(_) => panic!("Error reading file: {}", filename),
    };

    let mut counter = 0;
    for line in lines {
        let (first, last) = last_and_first_numeric(&line);
        counter += first * 10 + last;
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_and_first_numeric() {
        assert_eq!(last_and_first_numeric("one23two"), (1, 2));
    }

    #[test]
    fn test_last_and_first_numeric_2() {
        assert_eq!(last_and_first_numeric("5sixseven"), (5, 7));
    }

    #[test]
    fn test_last_and_first_numeric_3() {
        assert_eq!(last_and_first_numeric("5fivefive"), (5, 5));
    }

    #[test]
    fn test_last_and_first_numeric_5() {
        assert_eq!(last_and_first_numeric("six777nine"), (6, 9));
    }

    #[test]
    fn test_last_and_first_numeric_4() {
        assert_eq!(last_and_first_numeric("uvw"), (0, 0));
    }

    #[test]
    fn test_last_and_first_numeric_6() {
        assert_eq!(last_and_first_numeric("7pqrstsixteen"), (7, 6));
    }
}
