use crate::day10::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut idx = 0;
    let mut cycle: isize = 0;
    let mut x: isize = 1;
    let mut signal_strength = 0;
    let mut out = "".to_string();

    let mut cycles_left_in_instr = match input[idx] {
        super::input::Instr::Addx(val) => 2,
        super::input::Instr::Noop => 1,
    };

    loop {
        if cycles_left_in_instr > 0 {
            cycles_left_in_instr -= 1;

            cycle += 1;

            let hori_pos = (cycle - 1) % 40;

            let add = if (x - hori_pos).abs() <= 1 { "#" } else { "." };
            // println!("{} = {} => {}", cycle, x, add);
            out += add;

            if cycle % 40 == 0 {
                out += "\n";
            }
        } else {
            // println!("executing {:?}", input[idx]);
            match input[idx] {
                super::input::Instr::Addx(val) => x += val,
                super::input::Instr::Noop => {}
            }

            idx += 1;

            if idx == input.len() {
                break;
            }

            cycles_left_in_instr = match input[idx] {
                super::input::Instr::Addx(val) => 2,
                super::input::Instr::Noop => 1,
            }
        }
    }

    out.into()
}
