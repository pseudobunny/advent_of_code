use anyhow::{anyhow, Ok, Result};

use crate::parsing::get_number_line;

#[derive(Debug)]
pub struct History {
    analysis: Vec<Vec<i64>>,
}

impl History {
    fn new(line: &mut &str) -> Result<History> {
        let mut analysis = vec![];

        let history_values = get_number_line(line)?;
        analysis.push(history_values);

        while analysis
            .last()
            .ok_or(anyhow!("analysis vector empty"))?
            .iter()
            .any(|&i| i != 0)
        {
            let next_analysis = analysis
                .last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<i64>>();

            if next_analysis.len() < 1 {
                analysis.push(vec![0]);
            } else {
                analysis.push(next_analysis);
            }
        }

        Ok(History { analysis })
    }

    fn predict(&self) -> i64 {
        self.analysis
            .iter()
            .filter_map(|sub_analysis| sub_analysis.last())
            .sum()
    }

    fn predict_back(&self) -> i64 {
        self.analysis
            .iter()
            .rev()
            .filter_map(|sub_analysis| sub_analysis.first())
            .fold(0, |acc, n| n - acc)
    }
}

pub fn construct_histories(lines: &[String]) -> Result<Vec<History>> {
    let filtered_lines: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    filtered_lines
        .iter()
        .map(|line| History::new(&mut (line as &str)))
        .collect()
}

pub fn part_one_history_sum(histories: &[History]) -> i64 {
    histories.iter().map(|history| history.predict()).sum()
}

pub fn part_two_history_sum(histories: &[History]) -> i64 {
    histories.iter().map(|history| history.predict_back()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    ";

    fn lines() -> Vec<String> {
        TEST_INPUT.split("\n").map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part_one() {
        let histories = construct_histories(&lines()).unwrap();
        assert_eq!(part_one_history_sum(&histories), 114);
    }

    #[test]
    fn test_part_two() {
        let histories = construct_histories(&lines()).unwrap();
        assert_eq!(part_two_history_sum(&histories), 2);
    }
}
