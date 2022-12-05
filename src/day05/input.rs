use nom::{
    bytes::streaming::tag,
    character::streaming::{digit0, digit1},
    error::{Error, ErrorKind},
    sequence::tuple,
    IResult,
};

use crate::day05::Input;

const INPUT: &str = include_str!("../../input/05/input.txt");

fn parse_initial_state(state: &str) -> Vec<Vec<char>> {
    let max_stack_height = state.lines().count();
    let stack_count = (state.lines().next().unwrap().len() + 1) / 4;
    let line_len = state.lines().next().unwrap().len();

    let mut stacks = vec![Vec::with_capacity(max_stack_height); stack_count];
    let state_bytes = state.as_bytes();

    for stack in 0..stacks.len() {
        for height in 1..max_stack_height {
            let val = state_bytes[stack * 4 + 1 + (line_len + 2) * (max_stack_height - height - 1)];
            if val != b' ' {
                stacks[stack].push(val as char);
            } else {
                break;
            }
        }
    }

    stacks
}

fn parse_move(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, origin) = digit1(input)?;
    let (dest, _) = tag(" to ")(input)?;

    Ok((
        input,
        (
            count.parse().unwrap(),
            origin.parse().unwrap(),
            dest.parse().unwrap(),
        ),
    ))
}

fn parse_moves(moves: &str) -> Vec<(usize, usize, usize)> {
    moves
        .lines()
        .map(|line| parse_move(line).unwrap().1)
        .collect()
}

pub fn read() -> Input {
    let (initial, moves) = INPUT.split_once("\r\n\r\n").unwrap();

    (parse_initial_state(initial), parse_moves(moves))
}
