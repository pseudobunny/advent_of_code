use anyhow::{anyhow, Result};
use winnow::{
    ascii::multispace0,
    combinator::{repeat, terminated},
    token::take_while,
    PResult, Parser,
};

fn parse_digits(input: &mut &str) -> PResult<i64> {
    take_while(1.., (('0'..='9'), '-'))
        .parse_to()
        .parse_next(input)
}

fn parse_number_line(input: &mut &str) -> PResult<Vec<i64>> {
    repeat(0.., terminated(parse_digits, multispace0)).parse_next(input)
}

pub fn get_number_line<'s>(input: &mut &'s str) -> Result<Vec<i64>> {
    parse_number_line
        .parse_next(input)
        .map_err(|e| anyhow!(e.to_string()))
}
