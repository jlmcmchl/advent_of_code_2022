use crate::day18::Input;

const INPUT: &str = include_str!("../../input/18/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| {
            let mut iter = line.split(',').map(|val| val.parse().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect()
}
