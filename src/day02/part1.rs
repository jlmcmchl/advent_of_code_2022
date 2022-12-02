use crate::day02::{input::MatchResult, Input, Output};

pub fn solve(input: &Input) -> Output {
    Output::USize(
        input
            .iter()
            .map(|(opp, you)| you.score() + MatchResult::decide(you, opp).score())
            .sum(),
    )
}
