use crate::day13::{Input, Output};

use super::input::Signal;

pub fn solve(input: &Input) -> Output {
    let two = Signal::List(vec![Signal::List(vec![Signal::Value(2)])]);
    let six = Signal::List(vec![Signal::List(vec![Signal::Value(6)])]);
    let mut input = input.clone();
    input.push(two.clone());
    input.push(six.clone());

    input.sort_unstable();

    input
        .iter()
        .enumerate()
        .filter(|(_, signal)| two.eq(signal) || six.eq(signal))
        .map(|(id, _)| id + 1)
        .product::<usize>()
        .into()
}
