use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

#[derive(PartialEq, Eq, Debug)]
enum SchematicParts {
    Number(u32),
    Symbol(bool),
    Empty,
}

fn lines_to_matrix(lines: &[String]) -> Vec<Vec<SchematicParts>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c.is_numeric() {
                        SchematicParts::Number(c.to_digit(10).unwrap_or(0))
                    } else if c == '.' {
                        SchematicParts::Empty
                    } else {
                        SchematicParts::Symbol(c == '*')
                    }
                })
                .collect()
        })
        .collect()
}

fn generate_symbol_fields(schematic: &[Vec<SchematicParts>]) -> (Vec<Vec<bool>>, Vec<Vec<usize>>) {
    let height = schematic.len();
    let width = schematic.first().map_or(0, |line| line.len());

    let mut symbol_fields: Vec<_> = (0..height)
        .map(|_| (0..width).map(|_| false).collect::<Vec<_>>())
        .collect();

    let mut gear_fields: Vec<_> = (0..height)
        .map(|_| (0..width).map(|_| 0).collect::<Vec<_>>())
        .collect();

    let mut gear_num = 0;
    for (i, schematic_line) in schematic.iter().enumerate().take(height - 1).skip(1) {
        for (j, schematic_part) in schematic_line.iter().enumerate().take(width - 1).skip(1) {
            let gear = match schematic_part {
                SchematicParts::Number(_) | SchematicParts::Empty => continue,
                SchematicParts::Symbol(gear) => gear,
            };

            symbol_fields[i - 1][j - 1] = true;
            symbol_fields[i][j - 1] = true;
            symbol_fields[i + 1][j - 1] = true;
            symbol_fields[i - 1][j] = true;
            symbol_fields[i][j] = true;
            symbol_fields[i + 1][j] = true;
            symbol_fields[i - 1][j + 1] = true;
            symbol_fields[i][j + 1] = true;
            symbol_fields[i + 1][j + 1] = true;

            if *gear {
                gear_num += 1;

                gear_fields[i - 1][j - 1] = gear_num;
                gear_fields[i][j - 1] = gear_num;
                gear_fields[i + 1][j - 1] = gear_num;
                gear_fields[i - 1][j] = gear_num;
                gear_fields[i][j] = gear_num;
                gear_fields[i + 1][j] = gear_num;
                gear_fields[i - 1][j + 1] = gear_num;
                gear_fields[i][j + 1] = gear_num;
                gear_fields[i + 1][j + 1] = gear_num;
            }
        }
    }

    (symbol_fields, gear_fields)
}

fn digits_to_num(digits: &[u32]) -> u32 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| digit * 10_u32.pow(i as u32))
        .sum()
}

fn part_number_sum_part_one(schematic: &[Vec<SchematicParts>], symbol_fields: &[Vec<bool>]) -> u32 {
    schematic
        .iter()
        .enumerate()
        .flat_map(|(i, schematic_line)| {
            let mut part_numbers = vec![];

            let mut touching_symbol = false;
            let mut digit_collector = vec![];
            for (j, schematic_part) in schematic_line
                .iter()
                .chain([&SchematicParts::Empty])
                .enumerate()
            {
                if let SchematicParts::Number(digit) = schematic_part {
                    digit_collector.push(*digit);
                    touching_symbol |= symbol_fields[i][j];
                } else if !digit_collector.is_empty() {
                    if touching_symbol {
                        part_numbers.push(digits_to_num(&digit_collector));
                    }

                    touching_symbol = false;
                    digit_collector = vec![];
                }
            }

            part_numbers
        })
        .sum()
}

fn part_number_sum_part_two(schematic: &[Vec<SchematicParts>], gear_fields: &[Vec<usize>]) -> u32 {
    schematic
        .iter()
        .enumerate()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, Vec<u32>>, (i, schematic_line)| {
                let mut touching_gears = HashSet::new();
                let mut digit_collector = vec![];

                for (j, schematic_part) in schematic_line
                    .iter()
                    .chain([&SchematicParts::Empty])
                    .enumerate()
                {
                    if let SchematicParts::Number(digit) = schematic_part {
                        digit_collector.push(*digit);
                        let gear = gear_fields[i][j];
                        if gear != 0 {
                            touching_gears.insert(gear);
                        }
                    } else if !digit_collector.is_empty() {
                        let num = digits_to_num(&digit_collector);

                        for gear in touching_gears {
                            if let Some(num_vec) = acc.get_mut(&gear) {
                                num_vec.push(num)
                            } else {
                                acc.insert(gear, vec![num]);
                            }
                        }

                        touching_gears = HashSet::new();
                        digit_collector = vec![];
                    }
                }

                acc
            },
        )
        .iter()
        .filter_map(|(_, connected_nums)| {
            if connected_nums.len() == 2 {
                Some(connected_nums.iter().fold(1, |acc, n| acc * n))
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .expect("A filename must be passed as an argument.");

    let file_lines = lines_from_file(&filename);
    let schematic = lines_to_matrix(&file_lines);

    let (symbol_fields, gear_fields) = generate_symbol_fields(&schematic);

    println!("Part 1: {}", part_number_sum_part_one(&schematic, &symbol_fields));
    println!("Part 2: {}", part_number_sum_part_two(&schematic, &gear_fields))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_number_sum_part_one() {
        let lines = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let schematic = lines_to_matrix(&lines);

        let (symbol_fields, _) = generate_symbol_fields(&schematic);

        assert_eq!(part_number_sum_part_one(&schematic, &symbol_fields), 4361)
    }

    #[test]
    fn test_part_number_sum_part_two() {
        let lines = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let schematic = lines_to_matrix(&lines);

        let (_, gear_fields) = generate_symbol_fields(&schematic);

        assert_eq!(part_number_sum_part_two(&schematic, &gear_fields), 467835)
    }
}
