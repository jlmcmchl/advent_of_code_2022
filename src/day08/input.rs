use ndarray::Array;

use crate::day08::Input;

const INPUT: &str = include_str!("../../input/08/input.txt");

pub fn read() -> Input {
    Array::from_shape_vec(
        // [5, 5],
        [99, 99],
        INPUT
            .bytes()
            .filter(|b| *b != b'\n' && *b != b'\r')
            .map(|tree| tree - b'0')
            .collect(),
    )
    .unwrap()
}
