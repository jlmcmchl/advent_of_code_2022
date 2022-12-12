use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::{digit1, line_ending, multispace1},
    combinator::map,
    multi::{many_m_n, separated_list1},
    IResult,
};

use crate::day11::Input;

const INPUT: &str = include_str!("../../input/11/input.txt");

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<usize>,
    func: Func,
    pub test: Test,
}

impl Monkey {
    pub fn process(&self, items: &[usize], divide_worry: bool) -> Vec<(usize, usize)> {
        let res = items
            .iter()
            .map(|val| self.func.apply(*val))
            .map(|val| if divide_worry { val / 3 } else { val })
            .map(|val| self.test.check(val))
            .collect();

        res
    }

    pub fn add(&mut self, val: usize) {
        self.items.push(val);
    }
}

#[derive(Debug, Clone)]
struct Func {
    op: Op,
    right: Option<usize>,
}

impl Func {
    fn apply(&self, val: usize) -> usize {
        match self.op {
            Op::Add => val + self.right.unwrap_or(val),
            Op::Mul => val * self.right.unwrap_or(val),
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Mul,
    Add,
}

#[derive(Debug, Clone)]
pub struct Test {
    pub divisor: usize,
    true_target: usize,
    false_target: usize,
}

impl Test {
    fn check(&self, val: usize) -> (usize, usize) {
        if val % self.divisor == 0 {
            (val, self.true_target)
        } else {
            (val, self.false_target)
        }
    }
}

fn parse_items(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), digit1)(input)?;
    Ok((
        input,
        items.iter().map(|item| item.parse().unwrap()).collect(),
    ))
}

fn parse_func(input: &str) -> IResult<&str, Func> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, op_str) = take(1usize)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, right) = alt((
        map(tag("old"), |_| None),
        map(digit1, |val: &str| Some(val.parse().unwrap())),
    ))(input)?;

    let op = match op_str {
        "*" => Op::Mul,
        "+" => Op::Add,
        _ => unreachable!(),
    };

    Ok((input, Func { op, right }))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, divisor) = digit1(input)?;
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("If true: throw to monkey ")(input)?;
    let (input, true_target) = digit1(input)?;
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("If false: throw to monkey ")(input)?;
    let (input, false_target) = digit1(input)?;

    Ok((
        input,
        Test {
            divisor: divisor.parse().unwrap(),
            true_target: true_target.parse().unwrap(),
            false_target: false_target.parse().unwrap(),
        },
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, items) = parse_items(input)?;
    let (input, _) = multispace1(input)?;

    let (input, func) = parse_func(input)?;
    let (input, _) = multispace1(input)?;

    let (input, test) = parse_test(input)?;

    Ok((
        input,
        Monkey {
            id: id.parse().unwrap(),
            items,
            func,
            test,
        },
    ))
}

pub fn read() -> Input {
    separated_list1(multispace1, parse_monkey)(INPUT).unwrap().1
}
