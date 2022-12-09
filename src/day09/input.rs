use crate::day09::Input;

const INPUT: &str = include_str!("../../input/09/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| {
            let (dir, count) = line.split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match dir {
                "R" => (super::Direction::Right, count),
                "L" => (super::Direction::Left, count),
                "U" => (super::Direction::Up, count),
                "D" => (super::Direction::Down, count),
                _ => unreachable!(),
            }
        })
        .collect()
}
