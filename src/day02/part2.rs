use crate::day02::{Input, Output};

use super::input::{Choice, MatchResult};

pub fn solve(input: &Input) -> Output {
    Output::USize(
        input
            .iter()
            .map(|(opp, res)| {
                let res = MatchResult::map(res);
                let you = Choice::results_against(&res, opp);

                you.score() + res.score()
            })
            .sum(),
    )
}
