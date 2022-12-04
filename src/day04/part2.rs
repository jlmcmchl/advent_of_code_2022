use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    Output::USize(
        input
            .iter()
            .filter(|assignment| {
                assignment[0].max(assignment[2]) <= assignment[1].min(assignment[3])
            })
            .count(),
    )
}
