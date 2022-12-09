pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

pub type Input = (HashMap<String, (usize, Vec<String>)>);

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

fn folder_size(folder: &str, hierarchy: &HashMap<String, (usize, Vec<String>)>) -> usize {
    let (total, subfolders) = &hierarchy[folder];
    let mut total = *total;

    for subfolder in subfolders {
        total += folder_size(subfolder, hierarchy);
    }

    total
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
