pub mod input;
pub mod part1;
pub mod part2;

use ndarray::Array2;

use crate::{Output, Part};

pub type Input = Array2<Node>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Node {
    Start,
    Path(u8),
    End,
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
