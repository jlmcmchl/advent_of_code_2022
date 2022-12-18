use std::collections::{BinaryHeap, HashSet};

use crate::day18::{Input, Output};

fn neighbors((x, y, z): &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    vec![
        (x - 1, *y, *z),
        (x + 1, *y, *z),
        (*x, y - 1, *z),
        (*x, y + 1, *z),
        (*x, *y, z - 1),
        (*x, *y, z + 1),
    ]
}

pub fn solve(input: &Input) -> Output {
    let mut faces = HashSet::new();

    let mut seen = HashSet::new();

    let min_x = input.iter().map(|(x, ..)| *x).min().unwrap();
    let max_x = input.iter().map(|(x, ..)| *x).max().unwrap();
    let min_y = input.iter().map(|(_, y, _)| *y).min().unwrap();
    let max_y = input.iter().map(|(_, y, _)| *y).max().unwrap();
    let min_z = input.iter().map(|(.., z)| *z).min().unwrap();
    let max_z = input.iter().map(|(.., z)| *z).max().unwrap();

    let mut queue = vec![(min_x - 1, min_y - 1, min_z - 1)]
        .into_iter()
        .collect::<BinaryHeap<_>>();

    while let Some(here) = queue.pop() {
        if seen.contains(&here) {
            continue;
        }

        neighbors(&here)
            .iter()
            .filter(|(x, y, z)| {
                min_x - 1 <= *x
                    && *x <= 1 + max_x
                    && min_y - 1 <= *y
                    && *y <= 1 + max_y
                    && min_z - 1 <= *z
                    && *z <= 1 + max_z
            })
            .for_each(|neighbor| {
                if input.contains(neighbor) {
                    faces.insert((here, *neighbor));
                } else {
                    queue.push(*neighbor);
                }
            });

        seen.insert(here);
    }

    faces.len().into()
}
