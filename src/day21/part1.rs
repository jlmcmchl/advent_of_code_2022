use crate::day21::{Input, Output};

fn resolve_value(eqn: &[super::Equation], value: &super::Value) -> usize {
    match value {
        super::Value::Direct(val) => *val,
        _ => unreachable!(),
    }
}

pub fn solve((map, equation): &Input) -> Output {
    let mut equation = equation.clone();

    let root = map["root"];

    let mut stack = vec![root];

    while let Some(idx) = stack.last() {
        match equation[*idx] {
            super::Equation::Expression(
                super::Value::Indirect(a),
                op,
                super::Value::Indirect(b),
            ) => {
                if let super::Equation::Singleton(super::Value::Direct(a)) = equation[a] {
                    if let super::Equation::Singleton(super::Value::Direct(b)) = equation[b] {
                        let result = match op {
                            super::Op::Add => a + b,
                            super::Op::Sub => a - b,
                            super::Op::Mul => a * b,
                            super::Op::Div => a / b,
                        };

                        equation[*idx] = super::Equation::Singleton(super::Value::Direct(result));
                    } else {
                        stack.push(b);
                    }
                } else {
                    stack.push(a);
                }
            }
            super::Equation::Singleton(value) => {
                stack.pop();
            }
            _ => unreachable!(),
        };
    }

    if let super::Equation::Singleton(super::Value::Direct(val)) = equation[root] {
        val.into()
    } else {
        0.into()
    }
}
