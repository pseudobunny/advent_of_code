use anyhow::{anyhow, Result};
use winnow::{
    ascii::alpha0,
    combinator::{delimited, separated_pair},
    PResult, Parser,
};

fn parse_path_pair<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    delimited('(', separated_pair(alpha0, ", ", alpha0), ')').parse_next(input)
}

fn parse_node_components<'s>(input: &mut &'s str) -> PResult<(&'s str, (&'s str, &'s str))> {
    (separated_pair(alpha0, " = ", parse_path_pair)).parse_next(input)
}

pub fn get_node_components<'s>(input: &mut &'s str) -> Result<(&'s str, (&'s str, &'s str))> {
    parse_node_components
        .parse_next(input)
        .map_err(|e| anyhow!(e.to_string()))
}
