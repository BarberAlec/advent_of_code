use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

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


pub fn scratchcards(filename: &str) -> u32 {
    let lines = match read_file_line_by_line(filename) {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut index = 1;
    let number_of_lines = lines.len() as u32;
    let mut card_counter: HashMap<u32, u32> = (1..=number_of_lines).map(|i: u32| (i, 1)).collect();

    for line in lines {
        let (my_nums, winners) = parse_line(&line);
        let count = count_matches(my_nums, winners);

        for i in index+1..=(index + count) {
            card_counter.insert(i, card_counter[&i] + card_counter[&index]);
        }

        index += 1;
    }
    card_counter.values().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scratchcards() {
        let result = scratchcards("aoc_4/test_data_2.txt");
        assert_eq!(result, 30);
    }
}
