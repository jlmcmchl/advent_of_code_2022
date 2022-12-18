use crate::day17::Input;

const INPUT: &str = include_str!("../../input/17/input.txt");

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Left,
    Right,
}

pub fn read() -> Input {
    INPUT
        .chars()
        .filter_map(|chr| match chr {
            '>' => Some(Movement::Right),
            '<' => Some(Movement::Left),
            _ => None,
        })
        .collect()
}
