use std::str::FromStr;

use nom::{
    branch::alt,
    character::{
        complete::{line_ending, space1},
        streaming::{char, one_of},
    },
    combinator::{eof, map, map_opt},
    error::ParseError,
    multi::{many0, separated_list0},
    sequence::{separated_pair, terminated},
    streaming::take,
    IResult,
};

use crate::day02::Input;

const INPUT: &str = include_str!("../../input/02/input.txt");

#[derive(Debug, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub enum MatchResult {
    Win,
    Lose,
    Draw,
}

impl MatchResult {
    pub fn decide(you: &Choice, opp: &Choice) -> Self {
        match you {
            Choice::Rock => match opp {
                Choice::Rock => Self::Draw,
                Choice::Paper => Self::Lose,
                Choice::Scissors => Self::Win,
            },
            Choice::Paper => match opp {
                Choice::Rock => Self::Win,
                Choice::Paper => Self::Draw,
                Choice::Scissors => Self::Lose,
            },
            Choice::Scissors => match opp {
                Choice::Rock => Self::Lose,
                Choice::Paper => Self::Win,
                Choice::Scissors => Self::Draw,
            },
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }

    pub fn map(you: &Choice) -> Self {
        match you {
            Choice::Rock => Self::Lose,
            Choice::Paper => Self::Draw,
            Choice::Scissors => Self::Win,
        }
    }
}

impl Choice {
    pub fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn results_against(result: &MatchResult, opponent: &Choice) -> Choice {
        match result {
            MatchResult::Lose => match opponent {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            MatchResult::Draw => match opponent {
                Self::Rock => Self::Rock,
                Self::Paper => Self::Paper,
                Self::Scissors => Self::Scissors,
            },
            MatchResult::Win => match opponent {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
        }
    }
}

fn parse_choice(input: &str) -> IResult<&str, Choice> {
    map_opt(one_of("ABCXYZ"), |chr| match chr {
        'A' | 'X' => Some(Choice::Rock),
        'B' | 'Y' => Some(Choice::Paper),
        'C' | 'Z' => Some(Choice::Scissors),
        _ => None,
    })(input)
}

pub fn read() -> Input {
    separated_list0(
        line_ending,
        separated_pair(parse_choice, space1, parse_choice),
    )(INPUT)
    .unwrap()
    .1
}
