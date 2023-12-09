use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

use anyhow::{anyhow, Result};
use winnow::{
    ascii::{digit1, multispace0},
    combinator::{preceded, repeat, terminated},
    PResult, Parser,
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

struct Race {
    time: u64,
    distance: u64,
}

fn construct_races(lines: &[String]) -> Result<(Vec<Race>, Race)> {
    let filtered_lines: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut time_line = filtered_lines[0];
    let times = preceded(("Time:", multispace0), parse_number_line)
        .parse_next(&mut time_line)
        .map_err(|e| anyhow!(e.to_string()))?;

    let mut distance_line = filtered_lines[1];
    let distances = preceded(("Distance:", multispace0), parse_number_line)
        .parse_next(&mut distance_line)
        .map_err(|e| anyhow!(e.to_string()))?;

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect();

    let big_time: String = times.iter().map(|t| t.to_string()).collect();
    let big_distance: String = distances.iter().map(|d| d.to_string()).collect();

    let big_race = Race { time: big_time.parse::<u64>()?, distance: big_distance.parse::<u64>()? };

    Ok((races, big_race))
}

fn parse_digits(input: &mut &str) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

fn parse_number_line(input: &mut &str) -> PResult<Vec<u64>> {
    repeat(0.., terminated(parse_digits, multispace0)).parse_next(input)
}

fn is_whole(n: f64) -> bool {
    (n - n.round()).abs() < 0.00001
}

fn winning_possibilities(race: &Race) -> u64 {
    let left = (race.time as f64) * 0.5;
    let right = ((race.time.pow(2) as f64) * 0.25 - race.distance as f64).sqrt();

    if right < 0_f64 {
        return 0;
    }

    let min_f = left - right;
    let max_f = left + right;

    let min = if is_whole(min_f) {
        min_f.round() as u64 + 1
    } else {
        min_f.ceil() as u64
    };

    let max = if is_whole(max_f) {
        max_f.round() as u64 - 1
    } else {
        max_f.floor() as u64
    };

    max - min + 1
}

fn winning_possibilities_product(races: &[Race]) -> u64 {
    races
        .iter()
        .fold(1, |acc, race| acc * winning_possibilities(race))
}

fn main() -> Result<()> {
    let filename = env::args()
        .skip(1)
        .next()
        .expect("A filename must be passed as an argument.");

    let file_lines = lines_from_file(&filename);
    let (races, big_race) = construct_races(&file_lines)?;

    println!("Part 1: {}", winning_possibilities_product(&races));
    println!("Part 2: {}", winning_possibilities(&big_race));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    Time:      7  15   30
    Distance:  9  40  200
    ";

    fn races() -> (Vec<Race>, Race) {
        let lines = TEST_INPUT
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        construct_races(&lines).unwrap()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(winning_possibilities_product(&races().0), 288)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(winning_possibilities(&races().1), 71503)
    }
}
