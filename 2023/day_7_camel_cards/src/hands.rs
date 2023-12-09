use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::parsing::get_hand_components;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Joker,
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(c: char, use_joker: bool) -> Option<Self> {
        match c {
            '0'..='9' => Some(Card::Number(c.to_digit(10).unwrap())),
            'T' => Some(Card::Number(10)),
            'J' => Some(if use_joker { Card::Joker } else { Card::Jack }),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u64,
}

fn get_hand_type(cards: &[Card]) -> HandType {
    let mut card_map = HashMap::new();
    for card in cards {
        let initial_amt = card_map.get(&card).unwrap_or(&0);
        card_map.insert(card, initial_amt + 1);
    }

    if let Some(joker_amt) = card_map.get(&Card::Joker) {
        let max_card_amt = card_map
            .iter()
            .filter(|(&card, _)| card != &Card::Joker)
            .max_by(|(_, amt_a), (_, amt_b)| amt_a.cmp(amt_b));
        if let Some((card, amt)) = max_card_amt {
            card_map.insert(card, amt + joker_amt);
            card_map.remove(&Card::Joker);
        }
    }

    let mut sorted_values = card_map.values().collect::<Vec<_>>();
    sorted_values.sort();
    sorted_values.reverse();

    match sorted_values[..] {
        [5, ..] => HandType::FiveOfAKind,
        [4, ..] => HandType::FourOfAKind,
        [3, 2, ..] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, ..] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn construct_hands(lines: &[String], use_joker: bool) -> Result<Vec<Hand>> {
    let filtered_lines: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    filtered_lines
        .iter()
        .map(|&(mut line)| {
            let parse_result = get_hand_components(&mut line);

            if let Ok((hand_str, bid)) = parse_result {
                let cards: Vec<_> = hand_str
                    .iter()
                    .filter_map(|c| Card::new(*c, use_joker))
                    .collect();

                Ok(Hand {
                    hand_type: get_hand_type(&cards),
                    cards,
                    bid,
                })
            } else {
                Err(anyhow!("could not parse hand"))
            }
        })
        .collect()
}

pub fn part_one_total_winnings(lines: &[String]) -> Result<u64> {
    let mut hands = construct_hands(lines, false)?;
    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum();

    Ok(sum)
}

pub fn part_two_total_winnings(lines: &[String]) -> Result<u64> {
    let mut hands = construct_hands(lines, true)?;
    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    ";

    fn lines() -> Vec<String> {
        TEST_INPUT.split("\n").map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one_total_winnings(&lines()).unwrap(), 6440)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two_total_winnings(&lines()).unwrap(), 5905)
    }
}