use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_file_line_by_line(filename: &str) -> io::Result<Vec<String>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // discard all characters on each line before ":"
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines = lines.iter().map(|s| s.split(":").collect::<Vec<&str>>()[1].to_string()).collect();
    Ok(lines)
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut line_split = line.split("|");

    let my_nums = line_split.next().unwrap();
    let winners = line_split.next().unwrap();

    // convert to vector of u32
    let my_nums_vec = my_nums.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    let winners_vec = winners.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>();

    let my_nums = my_nums_vec.iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let winners = winners_vec.iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
 
    (my_nums, winners)
}

fn count_matches(my_nums: Vec<u32>, winners: Vec<u32>) -> u32 {
    let mut count = 0;
    for my_num in my_nums {
        for winner in &winners {
            if my_num == *winner {
                count += 1;
            }
        }
    }
    count
}

fn match_count_to_points(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    return u32::pow(2, count - 1);
}

pub fn scratchcards() -> u32 {
    let filename = "aoc_4/data.txt";
    let lines = match read_file_line_by_line(filename) {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut point_count = 0;
    for line in lines {
        let (my_nums, winners) = parse_line(&line);
        let count = count_matches(my_nums, winners);
        let points = match_count_to_points(count);
        point_count += points;
    }
    point_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_line_by_line() {
        let filename = "aoc_4/test_data.txt";
        let expected = vec![
            String::from(" 58 96 35 20 93 34 10 27 37 30 | 99 70 93 11 63 41 37 29  7 28 34 10 40 96 38 35 27 30 20 21  4 51 58 39 56"),
            String::from(" 64 84 57 46 53 86 90 99 59 70 | 99 59 30 83 84 70 31 57  6 29 18 82 15 88 86 53 51 64 32 47 44 46 80 39 90"),
        ];
        let result = read_file_line_by_line(filename).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line() {
        let line = "1 2 3 | 4 5 6";
        let expected = (vec![1, 2, 3], vec![4, 5, 6]);
        let result = parse_line(line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line_2() {
        let line = "11 222 33 | 41 51 6 7 8 9";
        let expected = (vec![11, 222, 33], vec![41, 51, 6, 7, 8, 9]);
        let result = parse_line(line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_matches() {
        let my_nums = vec![1, 2, 3];
        let winners = vec![1, 2, 3];
        let expected = 3;
        let result = count_matches(my_nums, winners);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_matches_2() {
        let my_nums = vec![1, 2, 3];
        let winners = vec![1, 2, 3, 4, 5, 6];
        let expected = 3;
        let result = count_matches(my_nums, winners);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_matches_3() {
        let my_nums = vec![41, 48, 83, 86, 17];
        let winners = vec![83, 86, 6, 31, 17, 9, 48, 53];
        let expected = 4;
        let result = count_matches(my_nums, winners);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_count_to_points() {
        let count = 0;
        let expected = 0;
        let result = match_count_to_points(count);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_count_to_points_2() {
        let count = 1;
        let expected = 1;
        let result = match_count_to_points(count);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_count_to_points_3() {
        let count = 2;
        let expected = 2;
        let result = match_count_to_points(count);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_count_to_points_4() {
        let count = 4;
        let expected = 8;
        let result = match_count_to_points(count);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_count_to_points_5() {
        let count = 3;
        let expected = 4;
        let result = match_count_to_points(count);
        assert_eq!(result, expected);
    }
}
