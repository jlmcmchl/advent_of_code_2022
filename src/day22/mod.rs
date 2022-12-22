pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Forward(usize),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn score(&self) -> usize {
        match self {
            Self::Left => 2,
            Self::Right => 0,
            Self::Up => 3,
            Self::Down => 1,
        }
    }
}

pub type Input = (HashMap<(usize, usize), bool>, Vec<Command>);

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
