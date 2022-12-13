use std::cmp::Ordering;

use crate::day13::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input
        .chunks(2)
        .enumerate()
        .map(|(id, signals)| match signals[0].cmp(&signals[1]) {
            Ordering::Less | Ordering::Equal => id + 1,
            Ordering::Greater => 0,
        })
        .sum::<usize>()
        .into()
}
