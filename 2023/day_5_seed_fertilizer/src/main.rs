use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

use anyhow::{anyhow, Result};
use winnow::{
    ascii::digit1,
    combinator::{preceded, separated},
    PResult, Parser,
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

#[derive(Debug)]
struct SeedsAndMaps {
    seeds: Vec<usize>,
    seed_to_soil: Vec<(usize, usize, usize)>,
    soil_to_fertilizer: Vec<(usize, usize, usize)>,
    fertilizer_to_water: Vec<(usize, usize, usize)>,
    water_to_light: Vec<(usize, usize, usize)>,
    light_to_temperature: Vec<(usize, usize, usize)>,
    temperature_to_humidity: Vec<(usize, usize, usize)>,
    humidity_to_location: Vec<(usize, usize, usize)>,
}

impl SeedsAndMaps {
    fn new(lines: &[String]) -> Result<SeedsAndMaps> {
        let mut lines_iter = lines.iter().map(|s| s.trim()).filter(|s| !s.is_empty());

        let seed_line_str = lines_iter.next().ok_or(anyhow!("no seed line"))?;
        let mut seed_line = seed_line_str.as_ref();
        let seeds = preceded("seeds: ", parse_number_line)
            .parse_next(&mut seed_line)
            .map_err(|e| anyhow!(e.to_string()))?;

        let mut mappings = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let mut current_mapping = 0;
        for mut line in lines_iter.skip(1) {
            if line.chars().next().is_some_and(|c| c.is_digit(10)) {
                if let [d, s, l] = parse_number_line.parse_next(&mut line).unwrap()[0..3] {
                    mappings[current_mapping].push((d, s, l));
                }
            } else {
                current_mapping += 1;
            }
        }

        Ok(SeedsAndMaps {
            seeds,
            seed_to_soil: mappings[0].clone(),
            soil_to_fertilizer: mappings[1].clone(),
            fertilizer_to_water: mappings[2].clone(),
            water_to_light: mappings[3].clone(),
            light_to_temperature: mappings[4].clone(),
            temperature_to_humidity: mappings[5].clone(),
            humidity_to_location: mappings[6].clone(),
        })
    }

    fn min_seed_range_location(&self) -> usize {
        self.seeds
            .windows(2)
            .step_by(2)
            .map(|window| match window {
                [start, len, ..] => (*start..start + len)
                    .map(|seed| self.map_seed(seed))
                    .min()
                    .unwrap_or(usize::MAX),
                _ => usize::MAX,
            })
            .min()
            .unwrap_or(usize::MAX)
    }

    fn min_seed_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.map_seed(*seed))
            .min()
            .unwrap_or(usize::MAX)
    }

    fn map_seed(&self, seed: usize) -> usize {
        let soil = map_input(seed, &self.seed_to_soil);
        let fertilizer = map_input(soil, &self.soil_to_fertilizer);
        let water = map_input(fertilizer, &self.fertilizer_to_water);
        let light = map_input(water, &self.water_to_light);
        let temperature = map_input(light, &self.light_to_temperature);
        let humidity = map_input(temperature, &self.temperature_to_humidity);

        map_input(humidity, &self.humidity_to_location)
    }
}

fn parse_digits(input: &mut &str) -> PResult<usize> {
    digit1.parse_to().parse_next(input)
}

fn parse_number_line(input: &mut &str) -> PResult<Vec<usize>> {
    separated(0.., parse_digits, " ").parse_next(input)
}

fn map_input(input: usize, mappings: &[(usize, usize, usize)]) -> usize {
    mappings
        .iter()
        .filter_map(|(d_start, s_start, len)| {
            if (*s_start..(s_start + len)).contains(&input) {
                Some(d_start + (input - s_start))
            } else {
                None
            }
        })
        .next()
        .unwrap_or(input)
}

fn main() -> Result<()> {
    let filename = env::args()
        .skip(1)
        .next()
        .expect("A filename must be passed as an argument.");

    let file_lines = lines_from_file(&filename);
    let seed_maps = SeedsAndMaps::new(&file_lines)?;

    println!("Part 1: {}", seed_maps.min_seed_location());
    println!("Part 2: {}", seed_maps.min_seed_range_location());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn test_part_one() {
        let test_lines: Vec<String> = TEST_INPUT.split("\n").map(|s| s.to_string()).collect();

        let seed_maps = SeedsAndMaps::new(&test_lines).unwrap();
        assert_eq!(seed_maps.min_seed_location(), 35);
    }

    #[test]
    fn test_part_two() {
        let test_lines: Vec<String> = TEST_INPUT.split("\n").map(|s| s.to_string()).collect();

        let seed_maps = SeedsAndMaps::new(&test_lines).unwrap();
        assert_eq!(seed_maps.min_seed_range_location(), 46);
    }
}
