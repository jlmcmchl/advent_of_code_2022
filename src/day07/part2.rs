use std::collections::HashMap;

use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let total_disk = 70000000;
    let used_space = super::folder_size("/", input);

    let free_space = total_disk - used_space;

    let space_to_free = 30000000 - free_space;

    input
        .keys()
        .map(|name| super::folder_size(name, input))
        .filter(|size| *size > space_to_free)
        .min()
        .unwrap()
        .into()
}
