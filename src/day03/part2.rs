use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    Output::U32(
        input
            .iter()
            .map(|(left, right)| left | right)
            .array_chunks::<3>()
            .map(|arr| (arr[0] & arr[1] & arr[2]).trailing_zeros() - 9)
            .sum(),
    )
}
