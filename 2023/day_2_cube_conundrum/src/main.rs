use std::{
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CubeColor {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for CubeColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color_amt = s.trim().split(' ').collect::<Vec<_>>();

        if color_amt.len() != 2 {
            return Err(anyhow!("Cannot parse {}", s));
        }

        let amount = color_amt[0].parse()?;

        match color_amt[1] {
            "blue" => Ok(CubeColor::Blue(amount)),
            "green" => Ok(CubeColor::Green(amount)),
            "red" => Ok(CubeColor::Red(amount)),
            _ => Err(anyhow!("Cannot parse {}", s)),
        }
    }
}

struct Game {
    id: u32,
    rounds: Vec<Vec<CubeColor>>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_rounds = s.split(':').collect::<Vec<_>>();

        if game_rounds.len() != 2 {
            return Err(anyhow!("Cannot parse {}", s));
        }

        let game = game_rounds[0].split(' ').collect::<Vec<_>>();

        if game.len() != 2 {
            return Err(anyhow!("Cannot parse {}", game_rounds[0]));
        }

        let id = game[1].parse()?;

        let rounds = game_rounds[1]
            .split(';')
            .map(|round_str| {
                round_str
                    .split(',')
                    .map(|color_str| color_str.parse::<CubeColor>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, rounds })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| {
            round.iter().all(|color| match color {
                CubeColor::Blue(n) => *n <= 14,
                CubeColor::Green(n) => *n <= 13,
                CubeColor::Red(n) => *n <= 12,
            })
        })
    }

    fn power(&self) -> u32 {
        let min_colors = self.rounds.iter().fold(
            [CubeColor::Red(0), CubeColor::Blue(0), CubeColor::Green(0)],
            |mut acc, colors| {
                colors.iter().for_each(|&color| {
                    match color {
                        CubeColor::Red(_) => acc[0] = std::cmp::max(color, acc[0]),
                        CubeColor::Blue(_) => acc[1] = std::cmp::max(color, acc[1]),
                        CubeColor::Green(_) => acc[2] = std::cmp::max(color, acc[2]),
                    };
                });

                acc
            },
        );

        min_colors.iter().fold(1, |acc, color| match color {
            CubeColor::Blue(n) => *n * acc,
            CubeColor::Green(n) => *n * acc,
            CubeColor::Red(n) => *n * acc,
        })
    }
}

fn possible_ids_sum(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|game| {
            if game.is_possible() {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn game_power_sum(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| game.power())
        .sum()
}

fn main() -> Result<()> {
    let filename = env::args()
        .skip(1)
        .next()
        .expect("A filename must be passed as an argument.");

    let file_lines = lines_from_file(&filename);
    let games: Vec<Game> = file_lines
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_>>()?;

    println!("Part One: {}", possible_ids_sum(&games));
    println!("Part Two: {}", game_power_sum(&games));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_possibility() {
        assert!("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<Game>()
            .unwrap()
            .is_possible());
        assert!(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
                .parse::<Game>()
                .unwrap()
                .is_possible()
        );
        assert!(
            !"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .parse::<Game>()
                .unwrap()
                .is_possible()
        );
        assert!(
            !"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                .parse::<Game>()
                .unwrap()
                .is_possible()
        );
        assert!("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .parse::<Game>()
            .unwrap()
            .is_possible());
    }

    #[test]
    fn test_possible_game_sum() {
        let lines = &[
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                .to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
        ];

        let games: Vec<Game> = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

        assert_eq!(
            possible_ids_sum(&games),
            8
        )
    }

    #[test]
    fn test_game_power() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
                .parse::<Game>()
                .unwrap()
                .power(),
            48
        );
        assert_eq!(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
                .parse::<Game>()
                .unwrap()
                .power(),
            12
        );
        assert_eq!(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .parse::<Game>()
                .unwrap()
                .power(),
            1560
        );
        assert_eq!(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                .parse::<Game>()
                .unwrap()
                .power(),
            630
        );
        assert_eq!(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .parse::<Game>()
                .unwrap()
                .power(),
            36
        );
    }

    #[test]
    fn test_game_power_sum() {
        let lines = &[
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                .to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
        ];

        let games: Vec<Game> = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
        
        assert_eq!(
            game_power_sum(&games),
            2286
        )
    }
}
