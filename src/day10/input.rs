use crate::day10::Input;

const INPUT: &str = include_str!("../../input/10/input.txt");

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Noop,
    Addx(isize),
}

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| {
            if let Some((_, rest)) = line.split_once(' ') {
                Instr::Addx(rest.parse().unwrap())
            } else {
                Instr::Noop
            }
        })
        .collect()
}
