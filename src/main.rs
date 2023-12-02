use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    day1()
}

fn day1() -> anyhow::Result<()> {
    println!(
        "day 1 part 1 answer: {}",
        day1_part1(puzzle_input_lines("input/day1.txt")?)
    );
    println!(
        "day 1 part 2 answer: {}",
        day1_part2(puzzle_input_lines("input/day1.txt")?)
    );
    Ok(())
}

fn day1_part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|line| {
            let is_digit = |c: &char| c.is_ascii_digit();
            let left_digit = line.chars().find(is_digit).unwrap() as u64 - '0' as u64;
            let right_digit = line.chars().rfind(is_digit).unwrap() as u64 - '0' as u64;
            10 * left_digit + right_digit
        })
        .sum()
}

fn day1_part2(input: impl Iterator<Item = String>) -> u64 {
    const DIGIT_NAMES_CHARS_AND_VALUES: [(&str, char, u64); 9] = [
        ("one", '1', 1),
        ("two", '2', 2),
        ("three", '3', 3),
        ("four", '4', 4),
        ("five", '5', 5),
        ("six", '6', 6),
        ("seven", '7', 7),
        ("eight", '8', 8),
        ("nine", '9', 9),
    ];
    input
        .map(|line| {
            let (_, _, left_digit) = DIGIT_NAMES_CHARS_AND_VALUES
                .iter()
                .min_by_key(|(name, char, _)| {
                    line.find(name)
                        .unwrap_or(usize::MAX)
                        .min(line.find(*char).unwrap_or(usize::MAX))
                })
                .unwrap();
            let (_, _, right_digit) = DIGIT_NAMES_CHARS_AND_VALUES
                .iter()
                .max_by_key(|(name, char, _)| {
                    let i32_score =
                        |o: Option<usize>| o.map(|i: usize| i as i32).unwrap_or(i32::MIN);
                    i32_score(line.rfind(name)).max(i32_score(line.rfind(*char)))
                })
                .unwrap();
            10 * left_digit + right_digit
        })
        .sum()
}

fn puzzle_input_lines(p: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    Ok(BufReader::new(File::open(p)?)
        .lines()
        .map_while(|line| line.ok()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Lines;
    use std::iter::MapWhile;

    type InputLinesIterator =
        MapWhile<Lines<BufReader<File>>, fn(io::Result<String>) -> Option<String>>;

    fn input_lines_for_test(input_file_path: &Path) -> InputLinesIterator {
        BufReader::new(File::open(input_file_path).unwrap())
            .lines()
            .map_while(|line| line.ok())
    }

    #[rstest]
    #[case(day1_part1, "input/day1.txt", 55108)]
    #[case(day1_part2, "input/day1.txt", 56324)]
    fn check_answers(
        #[case] part_function: fn(InputLinesIterator) -> u64,
        #[case] puzzle_input: impl AsRef<Path>,
        #[case] correct_answer: u64,
    ) {
        assert_eq!(
            part_function(input_lines_for_test(puzzle_input.as_ref())),
            correct_answer
        );
    }
}
