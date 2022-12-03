use nom::{
    character::streaming::{line_ending, not_line_ending},
    combinator::{consumed, recognize},
    multi::{many0, many1, separated_list0},
    IResult,
};

use crate::day03::Input;

const INPUT: &str = include_str!("../../input/03/input.txt");

fn encode_compartment(input: &str) -> u64 {
    input.chars().fold(0u64, |compartment, chr| {
        compartment | 1u64 << (chr.to_digit(36).unwrap() + if chr.is_uppercase() { 26 } else { 0 })
    })
}

fn parse_bag(input: &str) -> (u64, u64) {
    (
        encode_compartment(&input[..input.len() / 2]),
        encode_compartment(&input[input.len() / 2..]),
    )
}

pub fn read() -> Input {
    INPUT.lines().map(parse_bag).collect()
}
