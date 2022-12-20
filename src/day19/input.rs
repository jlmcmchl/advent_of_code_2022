use nom::{bytes::complete::tag, character::complete::digit1, IResult};

use crate::day19::Input;

use super::Blueprint;

const INPUT: &str = include_str!("../../input/19/input.txt");

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": Each ore robot costs ")(input)?;
    let (input, orebot_ore_cost) = digit1(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
    let (input, claybot_ore_cost) = digit1(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
    let (input, obsbot_ore_cost) = digit1(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsbot_clay_cost) = digit1(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
    let (input, geodebot_ore_cost) = digit1(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geodebot_obsidian_cost) = digit1(input)?;
    let (input, _) = tag(" obsidian.")(input)?;

    Ok((
        input,
        Blueprint {
            id: id.parse().unwrap(),
            orebot: orebot_ore_cost.parse().unwrap(),
            claybot: claybot_ore_cost.parse().unwrap(),
            obsidianbot: (
                obsbot_ore_cost.parse().unwrap(),
                obsbot_clay_cost.parse().unwrap(),
            ),
            geodebot: (
                geodebot_ore_cost.parse().unwrap(),
                geodebot_obsidian_cost.parse().unwrap(),
            ),
        },
    ))
}

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| parse_blueprint(line).unwrap().1)
        .collect()
}
