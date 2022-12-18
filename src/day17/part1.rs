use std::io::empty;

use lazy_static::lazy_static;
use ndarray::{array, Array2, Axis};

use crate::day17::{Input, Output};

#[derive(Debug, Clone)]
struct PlacedPiece<'a> {
    piece: &'a Piece,
    coord: (usize, usize),
}

#[derive(Debug, Default, Clone)]
struct Piece {
    shape: Array2<bool>,
}

lazy_static! {
    static ref PIECES: Vec<Piece> = vec![
        Piece {
            shape: array![[true, true, true, true]]
        },
        Piece {
            shape: array![
                [false, true, false],
                [true, true, true],
                [false, true, false]
            ]
        },
        Piece {
            shape: array![
                [true, true, true],
                [false, false, true],
                [false, false, true]
            ]
        },
        Piece {
            shape: array![[true], [true], [true], [true]]
        },
        Piece {
            shape: array![[true, true], [true, true]]
        },
    ];
    static ref EMPTY_ROW: Array2<bool> = array![[false, false, false, false, false, false, false]];
}

#[derive(Debug, Clone)]
struct Wind<'a> {
    input: &'a Input,
    idx: usize,
}

impl<'a> Wind<'a> {
    fn next(&mut self) -> super::input::Movement {
        let ret = self.input[self.idx];

        self.idx = (self.idx + 1) % self.input.len();

        ret
    }
}

fn apply(
    movement: super::input::Movement,
    coord: &(usize, usize),
    shape: &[usize],
) -> (usize, usize) {
    // println!("applying {movement:?} to piece of size {shape:?} at {coord:?}");

    if coord.1 == 0 && matches!(movement, super::input::Movement::Left)
        || coord.1 + shape[1] - 1 == 6 && matches!(movement, super::input::Movement::Right)
    {
        *coord
    } else {
        match movement {
            super::input::Movement::Left => (coord.0, coord.1 - 1),
            super::input::Movement::Right => (coord.0, coord.1 + 1),
        }
    }
}

fn height(tower: &Array2<bool>) -> usize {
    for i in (0..tower.shape()[0]).rev() {
        if (0..7).any(|j| tower[(i, j)]) {
            return i + 1;
        }
    }

    0
}

fn fits(tower: &Array2<bool>, coord: &(usize, usize), piece: &Piece) -> bool {
    let shape = piece.shape.shape();

    for i in 0..shape[0] {
        for j in 0..shape[1] {
            if piece.shape[(i, j)] && tower[(coord.0 + i, coord.1 + j)] {
                return false;
            }
        }
    }

    true
}

fn place_piece(tower: &mut Array2<bool>, coord: &(usize, usize), piece: &Piece) {
    let shape = piece.shape.shape();

    for i in 0..shape[0] {
        for j in 0..shape[1] {
            if piece.shape[(i, j)] {
                tower[(coord.0 + i, coord.1 + j)] = true;
            }
        }
    }
}

fn remove_piece(tower: &mut Array2<bool>, coord: &(usize, usize), piece: &Piece) {
    let shape = piece.shape.shape();

    for i in 0..shape[0] {
        for j in 0..shape[1] {
            if piece.shape[(i, j)] {
                tower[(coord.0 + i, coord.1 + j)] = false;
            }
        }
    }
}

fn place(piece: &Piece, wind: &mut Wind, tower: &mut Array2<bool>) {
    let tower_height = height(tower);

    for i in tower.shape()[0]..(tower_height + 3 + piece.shape.shape()[0]) {
        tower.append(Axis(0), EMPTY_ROW.view());
    }

    let shape = tower.shape();

    let mut coord = (tower_height + 3, 2);

    loop {
        let next = apply(wind.next(), &coord, piece.shape.shape());
        if fits(tower, &next, piece) {
            coord.1 = next.1;
        }

        if coord.0 == 0 || !fits(tower, &(coord.0 - 1, coord.1), piece) {
            // place
            place_piece(tower, &coord, piece);
            break;
        } else {
            coord.0 -= 1;
        }
    }
}

pub fn solve(input: &Input) -> Output {
    let mut wind = Wind { input, idx: 0 };

    let mut field = EMPTY_ROW.clone();

    for i in 0..2022 {
        place(&PIECES[i % PIECES.len()], &mut wind, &mut field);
    }

    height(&field).into()
}
