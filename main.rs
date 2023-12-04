mod aoc_1;
use aoc_1::part_1::trebuchet;

fn main(){
    let result = trebuchet();
    println!("Part 1 Result: {}", result);

    let result = aoc_1::part_2::trebuchet();
    println!("Part 2 Result: {}", result);
}
