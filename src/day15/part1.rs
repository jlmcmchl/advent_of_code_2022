use std::collections::HashSet;

use crate::day15::{Input, Output};

fn distance(sensor: &(isize, isize), beacon: &(isize, isize)) -> usize {
    sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)
}

pub fn solve(input: &Input) -> Output {
    let mut observable = HashSet::new();
    let row = 2000000; // 10;

    for (sensor, nearest_beacon) in input {
        let reach = distance(sensor, nearest_beacon);

        if sensor.1.abs_diff(row) > reach {
            continue;
        }

        let span = reach - sensor.1.abs_diff(row);
        println!("{sensor:?} can reach line {row} with a span of {span}");

        for extra in -(span as isize)..=(span as isize) {
            observable.insert((sensor.0 + extra, row));
        }
    }

    for (_, beacon) in input {
        if beacon.1 == row {
            observable.remove(beacon);
        }
    }

    observable.len().into()
}
