use std::collections::HashMap;

use crate::day11::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let monkeys = input;
    let mut monkey_inputs = monkeys
        .iter()
        .map(|monkey| monkey.items.clone())
        .collect::<Vec<_>>();

    let modulo: usize = monkeys.iter().map(|monkey| monkey.test.divisor).product();

    let mut inspections = vec![0; monkey_inputs.len()];

    for _ in 0..10000 {
        for i in 0..monkey_inputs.len() {
            let mut inputs = &mut monkey_inputs[i];
            let outputs = monkeys[i].process(inputs, false);

            inspections[i] += inputs.len();

            inputs.clear();

            for (val, target) in outputs {
                monkey_inputs[target].push(val % modulo);
            }
        }

        // for (i, inputs) in monkey_inputs.iter().enumerate() {
        //     println!("Monkey {i}: {inputs:?}");
        // }
    }

    inspections.sort();
    (inspections[inspections.len() - 1] * inspections[inspections.len() - 2]).into()
}
