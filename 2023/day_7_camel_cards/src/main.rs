use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

use anyhow::Result;

pub mod hands;
pub mod parsing;
use hands::{part_one_total_winnings, part_two_total_winnings};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

fn main() -> Result<()> {
    let filename = env::args()
        .skip(1)
        .next()
        .expect("A filename must be passed as an argument.");

    let file_lines = lines_from_file(&filename);

    println!("Part 1: {}", part_one_total_winnings(&file_lines)?);
    println!("Part 2: {}", part_two_total_winnings(&file_lines)?);

    Ok(())
}