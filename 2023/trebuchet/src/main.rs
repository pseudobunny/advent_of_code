use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

fn sum_calibration_values_part_one(lines: &[String]) -> u32 {
    println!("{}", lines.len());
    lines
        .iter()
        .map(|line| calibration_value_part_one(line))
        .sum()
}

fn calibration_value_part_one(line: &str) -> u32 {
    let mut numbers = line.chars().filter_map(|c| c.to_digit(10)).peekable();

    numbers.peek().unwrap_or(&0) * 10 + numbers.last().unwrap_or(0)
}

fn sum_calibration_values_part_two(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| calibration_value_part_two(line))
        .sum()
}

fn calibration_value_part_two(line: &str) -> u32 {
    let mut numbers = vec![];

    for (i, c) in line.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            numbers.push(d);
            continue;
        }

        for (j, num) in NUMBERS.iter().enumerate() {
            let possible_num = line
                .chars()
                .skip(i)
                .filter(|c2| c2.is_alphabetic())
                .take(num.len());

            if possible_num.clone().count() < num.len() {
                continue;
            }

            let is_match = possible_num.zip(num.chars()).all(|(a, b)| a == b);

            if is_match {
                numbers.push(j as u32 + 1)
            }
        }
    }

    numbers.first().unwrap_or(&0) * 10 + numbers.last().unwrap_or(&0)
}

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .expect("A filename must be passed as an argument.");

    let file_lines = lines_from_file(&filename);

    println!("Part One: {}", sum_calibration_values_part_one(&file_lines));
    println!("Part Two: {}", sum_calibration_values_part_two(&file_lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_value_part_one() {
        assert_eq!(calibration_value_part_one("1abc2"), 12);
        assert_eq!(calibration_value_part_one("pqr3stu8vwx"), 38);
        assert_eq!(calibration_value_part_one("a1b2c3d4e5f"), 15);
        assert_eq!(calibration_value_part_one("treb7uchet"), 77);
    }

    #[test]
    fn test_calibration_sum_part_one() {
        assert_eq!(
            sum_calibration_values_part_one(&[
                "1abc2".to_string(),
                "pqr3stu8vwx".to_string(),
                "a1b2c3d4e5f".to_string(),
                "treb7uchet".to_string()
            ]),
            142
        );
    }

    #[test]
    fn test_calibration_value_part_two() {
        assert_eq!(calibration_value_part_two("two1nine"), 29);
        assert_eq!(calibration_value_part_two("eightwothree"), 83);
        assert_eq!(calibration_value_part_two("abcone2threexyz"), 13);
        assert_eq!(calibration_value_part_two("xtwone3four"), 24);
        assert_eq!(calibration_value_part_two("4nineeightseven2"), 42);
        assert_eq!(calibration_value_part_two("zoneight234"), 14);
        assert_eq!(calibration_value_part_two("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_calibration_sum_part_two() {
        assert_eq!(
            sum_calibration_values_part_two(&[
                "two1nine".to_string(),
                "eightwothree".to_string(),
                "abcone2threexyz".to_string(),
                "xtwone3four".to_string(),
                "4nineeightseven2".to_string(),
                "zoneight234".to_string(),
                "7pqrstsixteen".to_string(),
            ]),
            281
        );
    }
}
