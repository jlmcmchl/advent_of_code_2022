use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list1,
    sequence::separated_pair, IResult,
};

use crate::day14::Input;

const INPUT: &str = include_str!("../../input/14/input.txt");

fn parse_pair(input: &str) -> IResult<&str, (usize, usize)> {
    let (inp, (a, b)) = separated_pair(digit1, tag(","), digit1)(input)?;

    Ok((inp, (a.parse().unwrap(), b.parse().unwrap())))
}

fn parse_line(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(tag(" -> "), parse_pair)(input)
}

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}
