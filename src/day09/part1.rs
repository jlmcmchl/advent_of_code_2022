use std::collections::HashSet;

use crate::day09::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut tail_seen = HashSet::new();

    let mut pose_vec = vec![[0, 0]; 2];

    tail_seen.insert(*pose_vec.last().unwrap());

    input.iter().for_each(|(dir, count)| {
        (0..*count).for_each(|_| {
            match dir {
                super::Direction::Right => pose_vec[0][0] += 1,
                super::Direction::Left => pose_vec[0][0] -= 1,
                super::Direction::Up => pose_vec[0][1] += 1,
                super::Direction::Down => pose_vec[0][1] -= 1,
            }

            for i in 1..pose_vec.len() {
                let motion = super::move_toward(&pose_vec[i], &pose_vec[i - 1]);
                pose_vec[i][0] += motion[0];
                pose_vec[i][1] += motion[1];
            }

            tail_seen.insert(*pose_vec.last().unwrap());
        });
    });

    tail_seen.len().into()
}
