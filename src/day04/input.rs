use std::ops::Range;

use crate::day04::Input;

const INPUT: &str = include_str!("../../input/04/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| {
            let mut iter = line
                .split(|chr| chr == ',' || chr == '-')
                .map(|idx| idx.parse::<usize>().unwrap());
            [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]
        })
        .collect()
}
