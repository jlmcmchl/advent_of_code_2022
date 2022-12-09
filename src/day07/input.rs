use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

use crate::day07::Input;

#[derive(Debug, Clone)]
pub enum Line {
    CD(String),
    LS,
    File(String, usize),
    Directory(String),
}

const INPUT: &str = include_str!("../../input/07/input.txt");

fn parse_cd(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = not_line_ending(input)?;

    Ok((input, Line::CD(dir.into())))
}

fn parse_ls(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ls")(input)?;

    Ok((input, Line::LS))
}

fn parse_dir(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = not_line_ending(input)?;

    Ok((input, Line::Directory(name.into())))
}

fn parse_file(input: &str) -> IResult<&str, Line> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;

    let (input, name) = not_line_ending(input)?;

    Ok((input, Line::File(name.into(), size.parse().unwrap())))
}

fn parse_input() -> Vec<Line> {
    let result = separated_list1(
        line_ending,
        alt((parse_cd, parse_ls, parse_dir, parse_file)),
    )(INPUT);

    result.unwrap().1
}

fn construct_filesystem(input: &[Line]) -> Input {
    let mut subfolders = HashMap::new();
    let mut stack = Vec::new();

    let mut idx = 0;

    while idx < input.len() {
        match &input[idx] {
            Line::CD(dir) => match dir.as_str() {
                "/" => {
                    stack.clear();
                    stack.push("/".into())
                }
                ".." => {
                    stack.pop();
                }
                dir => stack.push(dir.to_owned()),
            },
            Line::LS => {
                let mut content = Vec::new();
                let mut inner_idx = idx + 1;
                while inner_idx < input.len()
                    && matches!(&input[inner_idx], Line::File(..) | Line::Directory(..))
                {
                    if let Line::Directory(name) = &input[inner_idx] {
                        content.push(full_name(&stack) + "/" + name);
                    }
                    inner_idx += 1;
                }

                subfolders.insert(full_name(&stack), (0, content));
            }
            Line::Directory(name) => {
                // sizes.insert(full_name(&stack) + "/" + name, 0);
            }
            Line::File(name, size) => {
                // sizes.insert(full_name(&stack) + "/" + name, *size);

                subfolders
                    .entry(full_name(&stack))
                    .and_modify(|val| val.0 += *size);
            }
        }

        idx += 1;
    }

    subfolders
}

pub fn read() -> Input {
    let inp = parse_input();

    construct_filesystem(&inp)
}

fn full_name(stack: &[String]) -> String {
    stack.iter().cloned().reduce(|a, b| a + "/" + &b).unwrap()
}
