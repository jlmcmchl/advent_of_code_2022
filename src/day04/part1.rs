use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    Output::USize(
        input
            .iter()
            .filter(|assignment| {
                (assignment[0] <= assignment[2] && assignment[1] >= assignment[3])
                    || (assignment[0] >= assignment[2] && assignment[1] <= assignment[3])
            })
            .count(),
    )
}
