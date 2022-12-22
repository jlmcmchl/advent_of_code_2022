use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use crate::day21::Input;

const INPUT: &str = include_str!("../../input/21/input.txt");

struct EquationParser<'a> {
    reference: &'a HashMap<String, usize>,
}

impl<'a> EquationParser<'a> {
    fn parse_direct<'b>(&'a self, input: &'b str) -> IResult<&'b str, super::Value> {
        let (input, val) = digit1(input)?;

        Ok((input, super::Value::Direct(val.parse().unwrap())))
    }

    fn parse_indirect<'b>(&'a self, input: &'b str) -> IResult<&'b str, super::Value> {
        let (input, val) = alpha1(input)?;

        Ok((input, super::Value::Indirect(self.reference[val])))
    }

    fn parse_value<'b>(&'a self, input: &'b str) -> IResult<&'b str, super::Value> {
        alt((
            |input| self.parse_direct(input),
            |input| self.parse_indirect(input),
        ))(input)
    }

    fn parse_op<'b>(&'a self, input: &'b str) -> IResult<&'b str, super::Op>
    where
        'a: 'b,
    {
        alt((
            |input| self.parse_add(input),
            |input| self.parse_sub(input),
            |input| self.parse_mul(input),
            |input| self.parse_div(input),
        ))(input)
    }

    fn parse_add<'b>(&self, input: &'b str) -> IResult<&'b str, super::Op> {
        map(tag("+"), |_| super::Op::Add)(input)
    }

    fn parse_sub<'b>(&self, input: &'b str) -> IResult<&'b str, super::Op> {
        map(tag("-"), |_| super::Op::Sub)(input)
    }

    fn parse_mul<'b>(&self, input: &'b str) -> IResult<&'b str, super::Op> {
        map(tag("*"), |_| super::Op::Mul)(input)
    }

    fn parse_div<'b>(&self, input: &'b str) -> IResult<&'b str, super::Op> {
        map(tag("/"), |_| super::Op::Div)(input)
    }

    fn parse_expression<'b>(&'a self, input: &'b str) -> IResult<&'b str, super::Equation>
    where
        'a: 'b,
    {
        let (input, (first, _, op, _, second)) = tuple((
            |input| self.parse_value(input),
            tag(" "),
            |input| self.parse_op(input),
            tag(" "),
            |input| self.parse_value(input),
        ))(input)?;

        Ok((input, super::Equation::Expression(first, op, second)))
    }

    fn parse_equation<'b>(&'a self, input: &'b str) -> IResult<&'b str, super::Equation>
    where
        'a: 'b,
    {
        alt((
            |input| self.parse_expression(input),
            map(
                |input| self.parse_value(input),
                |val| super::Equation::Singleton(val),
            ),
        ))(input)
    }
}

pub fn read() -> Input {
    let simple_parse = INPUT
        .lines()
        .filter_map(|line| line.split_once(": "))
        .collect::<Vec<_>>();

    let map = simple_parse
        .iter()
        .enumerate()
        .map(|(idx, (name, _))| (name.to_string(), idx))
        .collect();

    let parser = EquationParser { reference: &map };

    let expressions = simple_parse
        .iter()
        .map(|(_, expr)| parser.parse_equation(expr).unwrap().1)
        .collect();

    (map, expressions)
}
