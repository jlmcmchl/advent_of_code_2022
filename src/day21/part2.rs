use std::collections::HashSet;

use crate::day21::{Input, Output};

pub fn solve((map, equation): &Input) -> Output {
    let mut seen = vec![false; equation.len()];
    let mut index = vec![""; equation.len()];

    map.iter().for_each(|(name, val)| index[*val] = name);

    let root = map["root"];

    let humn = map["humn"];

    println!("sleipnir::OptimizationProblem problem;");
    println!("auto humn = problem.DecisionVariable();");

    seen[humn] = true;

    while !seen[root] {
        equation.iter().enumerate().for_each(|(idx, eqn)| {
            if seen[idx] {
                return;
            }

            match eqn {
                super::Equation::Singleton(super::Value::Direct(value)) => {
                    println!("sleipnir::Variable {} = {value};", index[idx])
                }
                super::Equation::Expression(
                    super::Value::Indirect(a),
                    op,
                    super::Value::Indirect(b),
                ) => {
                    if !seen[*a] || !seen[*b] {
                        return;
                    }

                    match op {
                        super::Op::Add => {
                            println!("auto {} = {} + {};", index[idx], index[*a], index[*b])
                        }
                        super::Op::Sub => {
                            println!("auto {} = {} - {};", index[idx], index[*a], index[*b])
                        }
                        super::Op::Mul => {
                            println!("auto {} = {} * {};", index[idx], index[*a], index[*b])
                        }
                        super::Op::Div => {
                            println!("auto {} = {} / {};", index[idx], index[*a], index[*b])
                        }
                    }
                }
                _ => unreachable!(),
            }

            seen[idx] = true;
        });
    }

    let root_eqn = equation[root];
    match root_eqn {
        super::Equation::Expression(super::Value::Indirect(left), _, super::Value::Indirect(right)) => {
            println!("auto root_error = {} - {};", index[left], index[right]);
        },
        _ => unreachable!()
    }

    println!("problem.Minimize(root_error * root_error);");
    println!("problem.Solve();");
    println!("fmt::print(\"input part 2: {{}}\\n\", humn.Value(0));");

    0.into()
}
