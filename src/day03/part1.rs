use crate::day03::{Input, Output};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxysABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve(input: &Input) -> Output {
    // println!("{:#X?}", input);
    Output::U32(
        input
            .iter()
            .map(|(left, right)| (left & right).trailing_zeros() - 9)
            .sum(),
    )
}
