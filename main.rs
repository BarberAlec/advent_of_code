mod aoc_1;
use aoc_1::part_1::trebuchet;
mod aoc_4;
use aoc_4::part_1::scratchcards;

fn main(){
    let result = scratchcards();
    println!("result: {}", result);

    let result = aoc_4::part_2::scratchcards("aoc_4/data.txt");
    println!("result: {}", result);
}
