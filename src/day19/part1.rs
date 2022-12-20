use std::{collections::BinaryHeap, rc::Rc};

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
    // last: Option<Rc<State>>
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

    if state.ore - ore_allocated >= blueprint.orebot {
        let mut option = state.clone();
        option.orebots += 1;
        // option.last = Some(upstream.clone());

        options.push(option);
    }

    if state.ore - ore_allocated >= blueprint.claybot {
        let mut option = state.clone();
        option.claybots += 1;
        // option.last = Some(upstream.clone());

        options.push(option);
    }

    if state.ore - ore_allocated >= blueprint.obsidianbot.0
        && state.clay - clay_allocated >= blueprint.obsidianbot.1
    {
        let mut option = state.clone();
        option.obsidianbots += 1;
        // option.last = Some(upstream.clone());

        options.push(option);
    }

    if state.ore - ore_allocated >= blueprint.geodebot.0
        && state.obsidian - obsidian_allocated >= blueprint.geodebot.1
    {
        let mut option = state.clone();
        option.geodebots += 1;
        // option.last = Some(upstream.clone());

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
        if state.time == time_limit {
            results.push(state.clone());
            continue;
        }

        for mut option in options(blueprint, &state) {
            option.time += 1;
            option.ore += state.orebots;
            option.clay += state.claybots;
            option.obsidian += state.obsidianbots;
            option.geode += state.geodebots;

            queue.push(option);
        }

        state.time += 1;
        state.ore += state.orebots;
        state.clay += state.claybots;
        state.obsidian += state.obsidianbots;
        state.geode += state.geodebots;

        queue.push(state);
    }

    if let Some(best) = results.peek() {
        // let mut states = vec![best.clone()];
        // let mut current = best.clone();

        // while let Some(next) = current.last {
        //     states.push(next.as_ref().clone());
        //     current = next.as_ref().clone();
        // }

        // for state in states.iter().rev() {
        //     println!("{state:?}");
        // }

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
    dbg!(input);

    input.iter().map(quality).sum::<usize>().into()
}
