use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::{character::complete::digit1, IResult};

use crate::day22::Input;

const INPUT: &str = include_str!("../../input/22/input.txt");

fn parse_command(input: &str) -> IResult<&str, super::Command> {
    alt((
        map(digit1, |val: &str| {
            super::Command::Forward(val.parse().unwrap())
        }),
        map(tag("L"), |_| super::Command::Left),
        map(tag("R"), |_| super::Command::Right),
    ))(input)
}

fn parse_board(input: &str) -> IResult<&str, HashMap<(usize, usize), bool>> {
    let height = input.lines().count();
    let width = input.lines().map(|line| line.len()).max().unwrap();

    let board = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, chr)| *chr != ' ')
                .map(move |(col, chr)| match chr {
                    '.' => ((row + 1, col + 1), false),
                    '#' => ((row + 1, col + 1), true),
                    _ => unreachable!(),
                })
        })
        .collect::<HashMap<_, _>>();

    Ok(("", board))
}

pub fn read() -> Input {
    if let Some((board, commands)) = INPUT.split_once("\r\n\r\n") {
        let commands = many1(parse_command)(commands).unwrap().1;

        let map = parse_board(board).unwrap().1;

        (map, commands)
    } else {
        Default::default()
    }
}
