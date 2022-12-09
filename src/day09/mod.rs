pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = Vec<(Direction, usize)>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn move_toward(curr: &[isize; 2], dest: &[isize; 2]) -> [isize; 2] {
    let diff = [dest[0] - curr[0], dest[1] - curr[1]];

    if diff[0].abs() + diff[1].abs() > 2 {
        // move in both axes
        [diff[0].signum(), diff[1].signum()]
    } else if diff[0].abs() > 1 {
        // move tail left or right
        [diff[0].signum(), 0]
    } else if diff[1].abs() > 1 {
        // move tail up or down
        [0, diff[1].signum()]
    } else {
        [0, 0]
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
