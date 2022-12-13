use std::cmp::Ordering;

use nom::{
    branch::alt, bytes::streaming::tag, character::complete::digit1, multi::separated_list0,
    sequence::delimited, IResult,
};

use crate::day13::Input;

const INPUT: &str = include_str!("../../input/13/input.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signal {
    List(Vec<Signal>),
    Value(u8),
}

fn parse_value(input: &str) -> IResult<&str, Signal> {
    let (input, val) = digit1(input)?;

    Ok((input, Signal::Value(val.parse().unwrap())))
}

fn parse_list(input: &str) -> IResult<&str, Signal> {
    let (input, list) = delimited(
        tag("["),
        separated_list0(tag(","), alt((parse_list, parse_value))),
        tag("]"),
    )(input)?;

    Ok((input, Signal::List(list)))
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Signal::Value(val) => match other {
                Signal::Value(oth) => val.cmp(oth),
                _ => Signal::List(vec![self.clone()]).cmp(other),
            },
            Signal::List(list) => match other {
                Signal::Value(_) => self.cmp(&Signal::List(vec![other.clone()])),
                Signal::List(oth_list) => {
                    let mut i = 0;
                    while i < list.len() && i < oth_list.len() {
                        match list[i].cmp(&oth_list[i]) {
                            Ordering::Equal => {
                                i += 1;
                            }
                            els => {
                                return els;
                            }
                        }
                    }
                    if i == list.len() && i == oth_list.len() {
                        Ordering::Equal
                    } else if i == list.len() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            },
        }
    }
}

pub fn read() -> Input {
    INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_list(line).unwrap().1)
        .collect()
}
