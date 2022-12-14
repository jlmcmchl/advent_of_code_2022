use std::collections::HashSet;

use crate::day14::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let max_y = input
        .iter()
        .filter_map(|rocks| rocks.iter().map(|corner| corner.1).max())
        .max()
        .unwrap();

    let source = (500, 0);

    let mut grid = HashSet::new();

    for rock in input {
        for line in rock.array_windows::<2>() {
            let min_x = line[0].0.min(line[1].0);
            let max_x = line[0].0.max(line[1].0);

            let min_y = line[0].1.min(line[1].1);
            let max_y = line[0].1.max(line[1].1);
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid.insert((x, y));
                }
            }
        }
    }

    let mut sand_units = 0;

    let mut sand = source;

    loop {
        if sand.1 > max_y {
            break;
        }

        if !grid.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
            continue;
        }

        if !grid.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
            continue;
        }

        if !grid.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
            continue;
        }

        sand_units += 1;

        grid.insert(sand);

        sand = source;
    }

    sand_units.into()
}
