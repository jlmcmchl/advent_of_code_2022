pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

pub type Input = String;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

fn solve(input: &Input, scan: usize) -> Output {
    let input = input.as_bytes();
    let mut last_seen = HashMap::<u8, usize>::new();

    let mut idx = 0;
    let mut next_valid = 0;

    loop {
        let chr = input[idx];

        let last_idx = last_seen.get(&chr).unwrap_or(&0);

        if idx - last_idx >= scan && next_valid == idx {
            return (1 + idx).into();
        } else {
            next_valid = next_valid.max(idx + scan - (idx - last_idx));
        }

        last_seen.insert(chr, idx);

        idx += 1;
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
