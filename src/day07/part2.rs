use std::collections::HashMap;

use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut subfolders = HashMap::new();
    let mut parent = HashMap::new();
    let mut sizes = HashMap::new();
    let mut stack = Vec::new();

    sizes.insert("/".to_owned(), 0);

    let mut idx = 0;

    while idx < input.len() {
        match &input[idx] {
            super::Line::CD(dir) => match dir.as_str() {
                "/" => {
                    stack.clear();
                    stack.push("/".into())
                }
                ".." => {
                    stack.pop();
                }
                dir => stack.push(dir.to_owned()),
            },
            super::Line::LS => {
                let mut content = Vec::new();
                let mut inner_idx = idx + 1;
                while inner_idx < input.len()
                    && matches!(
                        &input[inner_idx],
                        super::Line::File(..) | super::Line::Directory(..)
                    )
                {
                    if let super::Line::Directory(name) = &input[inner_idx] {
                        content.push(full_name(&stack) + "/" + name);
                    }
                    inner_idx += 1;
                }

                subfolders.insert(full_name(&stack), content);
            }
            super::Line::Directory(name) => {
                sizes.insert(full_name(&stack) + "/" + name, 0);
                parent.insert(full_name(&stack) + "/" + name, full_name(&stack));
            }
            super::Line::File(name, size) => {
                sizes.insert(full_name(&stack) + "/" + name, *size);
                parent.insert(full_name(&stack) + "/" + name, full_name(&stack));

                sizes
                    .entry(full_name(&stack))
                    .and_modify(|val| *val += *size);
            }
        }

        idx += 1;
    }

    let total_disk = 70000000;
    let used_space = folder_size("/", &subfolders, &sizes);

    let free_space = total_disk - used_space;

    let space_to_free = 30000000 - free_space;

    subfolders
        .keys()
        .map(|name| folder_size(name, &subfolders, &sizes))
        .filter(|size| *size > space_to_free)
        .min()
        .unwrap()
        .into()
}

fn folder_size(
    folder: &str,
    hierarchy: &HashMap<String, Vec<String>>,
    sizes: &HashMap<String, usize>,
) -> usize {
    let mut total = sizes[folder];

    for subfolder in &hierarchy[folder] {
        total += folder_size(subfolder, hierarchy, sizes);
    }

    total
}

fn full_name(stack: &[String]) -> String {
    stack.iter().cloned().reduce(|a, b| a + "/" + &b).unwrap()
}
