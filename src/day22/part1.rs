use std::collections::HashMap;

use crate::day22::{Input, Output};

fn min_row_for_col(map: &HashMap<(usize, usize), bool>, col: usize) -> usize {
    map.iter()
        .filter(|((_, c), _)| *c == col)
        .map(|((row, _), _)| *row)
        .min()
        .unwrap()
}

fn max_row_for_col(map: &HashMap<(usize, usize), bool>, col: usize) -> usize {
    map.iter()
        .filter(|((_, c), _)| *c == col)
        .map(|((row, _), _)| *row)
        .max()
        .unwrap()
}

fn min_col_for_row(map: &HashMap<(usize, usize), bool>, row: usize) -> usize {
    map.iter()
        .filter(|((r, _), _)| *r == row)
        .map(|((_, col), _)| *col)
        .min()
        .unwrap()
}

fn max_col_for_row(map: &HashMap<(usize, usize), bool>, row: usize) -> usize {
    map.iter()
        .filter(|((r, _), _)| *r == row)
        .map(|((_, col), _)| *col)
        .max()
        .unwrap()
}

fn wrap_coord(
    map: &HashMap<(usize, usize), bool>,
    ((row, col), facing): &mut ((usize, usize), super::Direction),
) {
    match facing {
        super::Direction::Left => *col = max_col_for_row(map, *row),
        super::Direction::Right => *col = min_col_for_row(map, *row),
        super::Direction::Up => *row = max_row_for_col(map, *col),
        super::Direction::Down => *row = min_row_for_col(map, *col),
    }
}

fn apply(
    command: &super::Command,
    pose: &mut ((usize, usize), super::Direction),
    map: &HashMap<(usize, usize), bool>,
) {
    // dbg!(&command);
    // dbg!(&pose);
    match command {
        &super::Command::Forward(steps) => {
            for i in 0..steps {
                let mut next = match pose.1 {
                    super::Direction::Left => ((pose.0 .0, pose.0 .1 - 1), pose.1),
                    super::Direction::Right => ((pose.0 .0, pose.0 .1 + 1), pose.1),
                    super::Direction::Up => ((pose.0 .0 - 1, pose.0 .1), pose.1),
                    super::Direction::Down => ((pose.0 .0 + 1, pose.0 .1), pose.1),
                };
                if !map.contains_key(&next.0) {
                    wrap_coord(map, &mut next);
                }

                // dbg!(&next);

                if map[&next.0] {
                    return;
                } else {
                    *pose = next;
                }
            }
        }
        super::Command::Left => {
            pose.1 = match pose.1 {
                super::Direction::Left => super::Direction::Down,
                super::Direction::Right => super::Direction::Up,
                super::Direction::Up => super::Direction::Left,
                super::Direction::Down => super::Direction::Right,
            }
        }
        super::Command::Right => {
            pose.1 = match pose.1 {
                super::Direction::Left => super::Direction::Up,
                super::Direction::Right => super::Direction::Down,
                super::Direction::Up => super::Direction::Right,
                super::Direction::Down => super::Direction::Left,
            }
        }
    }
}

pub fn solve((map, commands): &Input) -> Output {
    let (position, _) = map
        .iter()
        .filter(|((row, col), blocked)| *row == 1 && !**blocked)
        .min_by_key(|((_, col), _)| col)
        .unwrap();

    let mut pose = (*position, super::Direction::Right);

    commands
        .iter()
        .for_each(|command| apply(command, &mut pose, map));

    dbg!(pose);

    (1000 * pose.0 .0 + 4 * pose.0 .1 + pose.1.score()).into()
}
