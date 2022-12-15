use crate::day15::Input;
use nom::branch::alt;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::recognize, sequence::pair,
    IResult,
};

const INPUT: &str = include_str!("../../input/15/input.txt");

fn parse_x(input: &str) -> IResult<&str, isize> {
    let (input, _) = tag("x=")(input)?;
    let (input, val) = alt((recognize(pair(tag("-"), digit1)), digit1))(input)?;

    Ok((input, val.parse().unwrap()))
}

fn parse_y(input: &str) -> IResult<&str, isize> {
    let (input, _) = tag("y=")(input)?;
    let (input, val) = alt((recognize(pair(tag("-"), digit1)), digit1))(input)?;

    Ok((input, val.parse().unwrap()))
}

fn parse_line(input: &str) -> IResult<&str, ((isize, isize), (isize, isize))> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, x0) = parse_x(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, y0) = parse_y(input)?;

    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, x1) = parse_x(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, y1) = parse_y(input)?;

    Ok((input, ((x0, y0), (x1, y1))))
}

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}
