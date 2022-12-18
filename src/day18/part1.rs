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
    input
        .iter()
        .map(|droplet| {
            neighbors(droplet)
                .iter()
                .filter(|neighbor| !input.contains(neighbor))
                .count()
        })
        .sum::<usize>()
        .into()
}
