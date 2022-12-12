use ndarray::Array2;

use crate::day12::Input;

const INPUT: &str = include_str!("../../input/12/input.txt");

pub fn read() -> Input {
    let mut line_ct = 2;
    let vec: Vec<super::Node> = INPUT
        .bytes()
        .filter_map(|c| {
            if c == b'S' {
                Some(super::Node::Start)
            } else if c == b'E' {
                Some(super::Node::End)
            } else if c >= b'a' {
                Some(super::Node::Path(c - b'a'))
            } else {
                line_ct += 1;
                None
            }
        })
        .collect();

    line_ct /= 2;

    let line_len = vec.len() / line_ct;

    println!("{} & {} => {}", vec.len(), line_ct, line_len);

    Array2::from_shape_vec([line_ct, line_len], vec).unwrap()
}
