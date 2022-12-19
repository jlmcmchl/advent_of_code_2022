use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::separated_list1,
    IResult,
};

use crate::day16::Input;

const INPUT: &str = include_str!("../../input/16/input.txt");

fn parse_line(input: &str) -> IResult<&str, (String, (usize, Vec<String>))> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, rate) = digit1(input)?;
    let (input, _) = tag("; tunnel")(input)?;
    let (input, _) = alt((tag(" leads to valve "), tag("s lead to valves ")))(input)?;
    let (input, tunnels) = separated_list1(tag(", "), alpha1)(input)?;

    let valve = valve.into();
    let rate = rate.parse().unwrap();
    let tunnels = tunnels.into_iter().map(|tunnel| tunnel.into()).collect();

    Ok((input, (valve, (rate, tunnels))))
}

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}
