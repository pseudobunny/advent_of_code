use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

struct Scratchcard {
    id: usize,
    winning_numbers: Vec<u32>,
    number_pool: HashSet<u32>,
}

impl FromStr for Scratchcard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card_id_nums = s.split(':').collect::<Vec<_>>();

        if card_id_nums.len() != 2 {
            return Err(anyhow!("Cannot parse {}", s));
        }

        let card_id = card_id_nums[0]
            .split(' ')
            .filter(|card_id_split| !card_id_split.is_empty())
            .collect::<Vec<_>>();

        if card_id.len() != 2 {
            return Err(anyhow!("Cannot parse {}", card_id_nums[0]));
        }

        let id = card_id[1].trim().parse()?;

        let nums = card_id_nums[1].split('|').collect::<Vec<_>>();
        if nums.len() != 2 {
            return Err(anyhow!("Cannot parse {}", card_id_nums[1]));
        }

        let winning_numbers = nums[0]
            .trim()
            .split(" ")
            .map(|n_str| n_str.trim())
            .filter(|n_str| !n_str.is_empty())
            .map(|n_str| n_str.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let number_pool = nums[1]
            .trim()
            .split(" ")
            .map(|n_str| n_str.trim())
            .filter(|n_str| !n_str.is_empty())
            .map(|n_str| n_str.parse())
            .collect::<Result<HashSet<_>, _>>()?;

        return Ok(Scratchcard {
            id,
            winning_numbers,
            number_pool,
        });
    }
}

impl Scratchcard {
    fn matches(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|num| self.number_pool.contains(num))
            .count()
    }
}

fn stack_winnings_part_one(cards: &[Scratchcard]) -> u32 {
    cards
        .iter()
        .map(|card| card.matches())
        .map(|matches| {
            1 * if matches > 0 {
                2_u32.pow(matches as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn stack_winnings_part_two(cards: &[Scratchcard]) -> u32 {
    let max_card_id = cards.len();

    let initial_stack = (1..=max_card_id)
        .map(|id| (id, 1))
        .collect::<HashMap<usize, u32>>();

    cards
        .iter()
        .fold(initial_stack, |mut acc, card| {
            let copy_count = *acc.get_mut(&card.id).unwrap_or(&mut 0);

            (1..=card.matches())
                .map(|i| card.id + i)
                .filter(|&copied_id| copied_id <= max_card_id)
                .for_each(|copied_id| {
                    let prev_copies = acc.get(&copied_id).unwrap_or(&0);
                    acc.insert(copied_id, prev_copies + copy_count);
                });

            acc
        })
        .into_values()
        .sum()
}

fn main() -> Result<()> {
    let filename = env::args()
        .skip(1)
        .next()
        .expect("A filename must be passed as an argument.");

    let file_lines = lines_from_file(&filename);
    let cards: Vec<Scratchcard> = file_lines
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_>>()?;

    println!("Part 1: {}", stack_winnings_part_one(&cards));
    println!("Part 2: {}", stack_winnings_part_two(&cards));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_winnings_part_one() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];

        let cards: Vec<Scratchcard> = lines
            .iter()
            .map(|line| line.parse::<Scratchcard>().unwrap())
            .collect();

        assert_eq!(stack_winnings_part_one(&cards), 13)
    }

    #[test]
    fn test_stack_winnings_part_two() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];

        let cards: Vec<Scratchcard> = lines
            .iter()
            .map(|line| line.parse::<Scratchcard>().unwrap())
            .collect();

        assert_eq!(stack_winnings_part_two(&cards), 30)
    }
}
