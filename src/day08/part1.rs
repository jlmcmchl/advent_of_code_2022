use crate::day08::{Input, Output};

fn is_visible(input: &Input, coord: &(usize, usize)) -> bool {
    let shape = input.shape();
    if coord.0 == 0 || coord.0 + 1 == shape[0] || coord.1 == 0 || coord.1 + 1 == shape[1] {
        return true;
    }

    let height = input[*coord];

    let mut visible = true;

    // check west
    for i in (0..coord.0).rev() {
        if input[(i, coord.1)] >= height {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;

    // check east
    for i in (coord.0 + 1)..shape[0] {
        if input[(i, coord.1)] >= height {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;

    // check north
    for i in (0..coord.1).rev() {
        if input[(coord.0, i)] >= height {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;

    // check south
    for i in (coord.1 + 1)..shape[1] {
        if input[(coord.0, i)] >= height {
            visible = false;
            break;
        }
    }

    visible
}

pub fn solve(input: &Input) -> Output {
    let shape = input.shape();

    (0..shape[0])
        .flat_map(|col| (0..shape[1]).map(move |row| (col, row)))
        .filter(|coord| is_visible(input, coord))
        .count()
        .into()
}
