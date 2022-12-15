use std::collections::HashSet;

use crate::day15::{Input, Output};

fn distance(sensor: &(isize, isize), beacon: &(isize, isize)) -> usize {
    sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)
}

pub fn solve(input: &Input) -> Output {
    let mut coord: usize = 0;
    let side_len = 4000001;

    while coord < side_len * side_len {
        let x = coord / side_len;
        let y = coord % side_len;

        if x % 4000 == 0 && y == 0 {
            println!("searching row {x}");
        }

        let skip = input
            .iter()
            .filter_map(|(sensor, beacon)| {
                let coord_dist = distance(sensor, &(x as isize, y as isize));
                let beacon_dist = distance(sensor, beacon);

                if coord_dist <= beacon_dist {
                    // println!("sensor {:?} can see no beacon at {:?} because it can see {:?}", sensor, (x, y), beacon);
                    Some(beacon_dist - coord_dist + 1)
                } else {
                    None
                }
            })
            .max();

        match skip {
            Some(skip) => {
                if y + skip > side_len - 1 {
                    coord = (x + 1) * side_len;
                } else {
                    coord += skip;
                }
            }
            None => {
                return (x * 4000000 + y).into();
            }
        }
    }

    0.into()
}
