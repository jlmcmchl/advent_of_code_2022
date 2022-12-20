use std::{collections::BinaryHeap, rc::Rc, result};

use rayon::prelude::*;

use crate::day19::{Input, Output};

use super::Blueprint;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct State {
    time: u8,
    orebots: u8,
    claybots: u8,
    obsidianbots: u8,
    geodebots: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl State {
    fn fast_forward(&self, dt: u8) -> Self {
        State {
            time: self.time + dt,
            ore: self.ore.saturating_add(dt.saturating_mul(self.orebots)),
            clay: self.clay.saturating_add(dt.saturating_mul(self.claybots)),
            obsidian: self
                .obsidian
                .saturating_add(dt.saturating_mul(self.obsidianbots)),
            geode: self.geode.saturating_add(dt.saturating_mul(self.geodebots)),
            orebots: self.orebots,
            claybots: self.claybots,
            obsidianbots: self.obsidianbots,
            geodebots: self.geodebots,
        }
    }

    fn allocated_ore(&self, blueprint: &Blueprint) -> u8 {
        blueprint.orebot * self.orebots
            + blueprint.claybot * self.claybots
            + blueprint.obsidianbot.0 * self.obsidianbots
            + blueprint.geodebot.0 * self.geodebots
    }

    fn allocated_clay(&self, blueprint: &Blueprint) -> u8 {
        blueprint.obsidianbot.1 * self.obsidianbots
    }

    fn allocated_obsidian(&self, blueprint: &Blueprint) -> u8 {
        blueprint.geodebot.1 * self.geodebots
    }

    fn time_to_ore(&self, blueprint: &Blueprint, ore: u8) -> u8 {
        if self.ore >= self.allocated_ore(blueprint) + ore {
            0
        } else {
            (ore - (self.ore - self.allocated_ore(blueprint))).div_ceil(self.orebots)
        }
    }

    fn time_to_clay(&self, blueprint: &Blueprint, clay: u8) -> u8 {
        if self.clay >= self.allocated_clay(blueprint) + clay {
            0
        } else {
            (clay - (self.clay - self.allocated_clay(blueprint))).div_ceil(self.claybots)
        }
    }

    fn time_to_obsidian(&self, blueprint: &Blueprint, obsidian: u8) -> u8 {
        if self.obsidian >= self.allocated_obsidian(blueprint) + obsidian {
            0
        } else {
            (obsidian - (self.obsidian - self.allocated_obsidian(blueprint)))
                .div_ceil(self.obsidianbots)
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.geode
            .cmp(&other.geode)
            .then(self.geodebots.cmp(&other.geodebots))
            // .then(self.obsidian.cmp(&other.obsidian))
            .then(self.obsidianbots.cmp(&other.obsidianbots))
            // .then(self.clay.cmp(&other.clay))
            .then(self.claybots.cmp(&other.claybots))
            // .then(self.ore.cmp(&other.ore))
            .then(self.orebots.cmp(&other.orebots))
    }
}

fn options(blueprint: &Blueprint, state: &State) -> Vec<State> {
    let mut options: Vec<State> = Vec::new();
    let upstream = Rc::new(state.clone());

    let ore_allocated = blueprint.orebot * state.orebots
        + blueprint.claybot * state.claybots
        + blueprint.obsidianbot.0 * state.obsidianbots
        + blueprint.geodebot.0 * state.geodebots;
    let clay_allocated = blueprint.obsidianbot.1 * state.obsidianbots;
    let obsidian_allocated = blueprint.geodebot.1 * state.geodebots;

    let max_orebots = blueprint
        .orebot
        .max(blueprint.claybot)
        .max(blueprint.obsidianbot.0)
        .max(blueprint.geodebot.0);

    let max_claybots = blueprint.obsidianbot.1;

    let max_obsidianbots = blueprint.geodebot.1;

    if state.orebots < max_orebots {
        // try making an orebot next
        let wait = state.time_to_ore(blueprint, blueprint.orebot);
        let mut option = state.fast_forward(wait + 1);
        option.orebots += 1;

        options.push(option);
    }

    if state.claybots < max_claybots {
        // try making a claybot next
        let wait = state.time_to_ore(blueprint, blueprint.claybot);
        let mut option = state.fast_forward(wait + 1);
        option.claybots += 1;

        options.push(option);
    }

    if state.claybots > 0 && state.obsidianbots < max_obsidianbots {
        // try making an obsidianbot
        let wait_ore = state.time_to_ore(blueprint, blueprint.obsidianbot.0);
        let wait_clay = state.time_to_clay(blueprint, blueprint.obsidianbot.1);
        let mut option = state.fast_forward(wait_ore.max(wait_clay) + 1);
        option.obsidianbots += 1;

        options.push(option);
    }

    if state.obsidianbots > 0 {
        // try making a geodebot
        let wait_ore = state.time_to_ore(blueprint, blueprint.geodebot.0);
        let wait_obsidian = state.time_to_obsidian(blueprint, blueprint.geodebot.1);
        let mut option = state.fast_forward(wait_ore.max(wait_obsidian) + 1);
        option.geodebots += 1;

        options.push(option);
    }

    options
}

fn max_geodes_by(blueprint: &Blueprint, time_limit: u8) -> u8 {
    let mut results: BinaryHeap<State> = BinaryHeap::new();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    queue.push(State {
        ore: blueprint.orebot,
        orebots: 1,
        ..Default::default()
    });

    while let Some(mut state) = queue.pop() {
        if state.time >= time_limit {
            continue;
        }

        for mut option in options(blueprint, &state) {
            queue.push(option);
        }

        if state.geodebots > 0 {
            results.push(state.fast_forward(time_limit - state.time));
        }
    }

    if let Some(best) = results.peek() {

        println!("Solved blueprint {blueprint:?}");
        println!("{best:?}");

        best.geode
    } else {
        0
    }
}

fn quality(blueprint: &Blueprint) -> usize {
    blueprint.id as usize * max_geodes_by(blueprint, 24) as usize
}

pub fn solve(input: &Input) -> Output {
    input.par_iter().map(quality).sum::<usize>().into()
}
