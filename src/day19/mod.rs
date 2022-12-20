pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blueprint {
    id: u8,
    orebot: u8,
    claybot: u8,
    obsidianbot: (u8, u8),
    geodebot: (u8, u8),
}

pub type Input = Vec<Blueprint>;

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
