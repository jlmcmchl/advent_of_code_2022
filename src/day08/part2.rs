use crate::day08::{Input, Output};

fn visibility_score(input: &Input, coord: &(usize, usize)) -> usize {
    let height = input[*coord];
    let shape = input.shape();

    let mut up_score = 0;

    for i in (0..coord.0).rev() {
        up_score += 1;
        if input[(i, coord.1)] >= height {
            break;
        }
    }

    let mut down_score = 0;

    for i in coord.0 + 1..shape[0] {
        down_score += 1;
        if input[(i, coord.1)] >= height {
            break;
        }
    }

    let mut left_score = 0;

    for i in (0..coord.1).rev() {
        left_score += 1;
        if input[(coord.0, i)] >= height {
            break;
        }
    }

    let mut right_score = 0;

    for i in coord.1 + 1..shape[1] {
        right_score += 1;
        if input[(coord.0, i)] >= height {
            break;
        }
    }

    up_score * down_score * left_score * right_score
}

pub fn solve(input: &Input) -> Output {
    let shape = input.shape();

    (1..shape[0] - 1)
        .flat_map(|col| (1..shape[1] - 1).map(move |row| (col, row)))
        .map(|coord| visibility_score(input, &coord))
        .max()
        .unwrap()
        .into()
}
