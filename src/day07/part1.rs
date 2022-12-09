use std::collections::HashMap;

use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input
        .keys()
        .map(|name| (name, super::folder_size(name, input)))
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum::<usize>()
        .into()
}
