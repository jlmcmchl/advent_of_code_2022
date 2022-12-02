use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    INPUT
        .replace('\r', "")
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|item| item.trim().parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}
