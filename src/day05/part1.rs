use crate::day05::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();

    moves.iter().for_each(|(count, origin, dest)| {
        for i in 0..*count {
            let tmp = stacks[*origin - 1].pop().unwrap();
            stacks[*dest - 1].push(tmp);
        }
    });

    Output::String(stacks.iter().map(|stack| stack.last().unwrap()).collect())
}
