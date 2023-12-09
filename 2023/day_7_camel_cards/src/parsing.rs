use anyhow::{anyhow, Result};
use winnow::{
    ascii::{digit1, multispace0},
    combinator::{repeat, terminated},
    token::one_of,
    PResult, Parser,
};

fn parse_digits(input: &mut &str) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

fn parse_card_str(input: &mut &str) -> PResult<char> {
    one_of(('0'..='9', 'A', 'K', 'Q', 'J', 'T')).parse_next(input)
}

fn parse_hand_str(input: &mut &str) -> PResult<Vec<char>> {
    repeat(1.., parse_card_str).parse_next(input)
}

fn parse_hand_components(input: &mut &str) -> PResult<(Vec<char>, u64)> {
    (terminated(parse_hand_str, multispace0), parse_digits).parse_next(input)
}

pub fn get_hand_components(input: &mut &str) -> Result<(Vec<char>, u64)> {
    parse_hand_components
        .parse_next(input)
        .map_err(|e| anyhow!(e.to_string()))
}
