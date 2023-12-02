use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    day1()
}

fn day1() -> anyhow::Result<()> {
    println!("day 1 part 1 answer: {}", day1_part1(puzzle_input_lines("input/day1.txt")?));
    println!("day 1 part 2 answer: {}", day1_part2(puzzle_input_lines("input/day1.txt")?));
    Ok(())
}

fn day1_part1(input: impl Iterator<Item = String>) -> u64 {
    input.map(|line| {
        let is_digit = |c: &char| c.is_ascii_digit();
        let first_digit = line.chars().find(is_digit).unwrap() as u64 - '0' as u64;
        let last_digit = line.chars().rfind(is_digit).unwrap() as u64 - '0' as u64;
        10 * first_digit + last_digit
    }).sum()
}

fn day1_part2(input: impl Iterator<Item = String>) -> i32 {
    input.count() as i32
}

fn puzzle_input_lines(p: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    Ok(BufReader::new(File::open(p)?)
        .lines()
        .map_while(|line| line.ok()))
}
