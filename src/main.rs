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

#[cfg(test)]
mod tests {
    use std::io::Lines;
    use std::iter::MapWhile;
    use super::*;
    use rstest::*;

    type InputLinesIterator = MapWhile<Lines<BufReader<File>>, fn(io::Result<String>) -> Option<String>>;

    fn input_lines_for_test(input_file_path: &Path) -> InputLinesIterator {
        BufReader::new(File::open(input_file_path).unwrap())
            .lines()
            .map_while(|line| line.ok())
    }

    #[rstest]
    #[case(day1_part1, "input/day1.txt", 55108)]
    fn check_answers(
        #[case] part_function: fn(InputLinesIterator) -> u64,
        #[case] puzzle_input: impl AsRef<Path>,
        #[case] correct_answer: u64
    ) {
        assert_eq!(part_function(input_lines_for_test(puzzle_input.as_ref())), correct_answer);
    }
}