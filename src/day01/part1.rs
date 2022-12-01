use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    return Output::USize(*input.iter().max().unwrap());
}
