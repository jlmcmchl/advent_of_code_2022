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
    let mut last_seen = 0u32;

    let mut idx = scan;

    for chr in &input[0..scan] {
        last_seen ^= 1 << (chr - b'a');
    }

    loop {
        if last_seen.count_ones() as usize == scan {
            return idx.into();
        }

        last_seen ^= 1 << (input[idx] - b'a');
        last_seen ^= 1 << (input[idx - scan] - b'a');

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
