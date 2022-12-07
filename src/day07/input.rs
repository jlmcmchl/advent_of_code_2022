use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

use crate::day07::Input;

use super::Line;

const INPUT: &str = include_str!("../../input/07/input.txt");

fn parse_cd(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = not_line_ending(input)?;

    Ok((input, Line::CD(dir.into())))
}

fn parse_ls(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ls")(input)?;

    Ok((input, Line::LS))
}

fn parse_dir(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = not_line_ending(input)?;

    Ok((input, Line::Directory(name.into())))
}

fn parse_file(input: &str) -> IResult<&str, Line> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;

    let (input, name) = not_line_ending(input)?;

    Ok((input, Line::File(name.into(), size.parse().unwrap())))
}

pub fn read() -> Input {
    let result = separated_list1(
        line_ending,
        alt((parse_cd, parse_ls, parse_dir, parse_file)),
    )(INPUT);

    result.unwrap().1
}
