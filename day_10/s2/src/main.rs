fn parse_input(input: Option<&str>) -> Vec<String> {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };
    let output = input
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|o| o.to_string())
        .collect::<Vec<String>>();

    output
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_line(input: Vec<String>) -> Vec<Instruction> {
    let instructions = input
        .iter()
        .map(|line| {
            let v = line.split(' ').collect::<Vec<&str>>();
            match v[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(v[1].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Instruction>>();
    instructions
}

fn check_register(tick: i32, register_x: &i32, signal: &mut i32) {
    dbg!(tick, &register_x);
    match tick {
        20 => {
            *signal += tick * register_x;
            dbg!(tick, &register_x, &signal);
        }
        60 => {
            *signal += tick * register_x;
            dbg!(tick, &register_x, &signal);
        }
        100 => {
            *signal += tick * register_x;
            dbg!(tick, &register_x, &signal);
        }
        140 => {
            *signal += tick * register_x;
            dbg!(tick, &register_x, &signal);
        }
        180 => {
            *signal += tick * register_x;
            dbg!(tick, &register_x, &signal);
        }
        220 => {
            *signal += tick * register_x;
            dbg!(tick, &register_x, &signal);
        }
        _ => (),
    }
}

fn draw_pixel(crt: &mut Vec<char>, register_x: &mut i32, tick: i32) {
    let sprite = *register_x - 1..=*register_x + 1;
    if tick == 0 {
        crt.push('\n');
    }

    if sprite.contains(&(tick)) {
        crt.push('#');
    } else {
        crt.push('.');
    }
}

fn run(input: Vec<String>) -> String {
    let instructions = parse_line(input);
    let mut register_x: i32 = 1;
    dbg!(&instructions);

    let mut ticks = (0..40).cycle();
    let mut instructions_counter = 0;
    let mut signal = 0;
    let mut crt: Vec<char> = Vec::new();

    loop {
        let mut tick = ticks.next().unwrap();
        check_register(tick, &register_x, &mut signal);
        draw_pixel(&mut crt, &mut register_x, tick);

        match instructions.get(instructions_counter) {
            Some(Instruction::Noop) => (),
            Some(Instruction::Addx(value)) => {
                tick = ticks.next().unwrap();
                check_register(tick, &register_x, &mut signal);
                draw_pixel(&mut crt, &mut register_x, tick);
                register_x += value;
            }
            None => {
                crt.pop();
                crt.remove(0);
                let output = crt
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                break output;
            }
        };
        instructions_counter += 1;
    }
    // signal
}

fn main() {
    let input = parse_input(None);

    let answer = run(input);

    println!("Answer: \n{}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            noop
            addx 3
            addx -5
            "
        )));
        dbg!(&input);
        // let answer = run(input);
        // assert_eq!(answer, 0);
    }

    #[test]
    // #[ignore = "reason"]
    fn test_run2() {
        let input = parse_input(Some(indoc!(
            "
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(
            answer,
            indoc!(
                "
                ##..##..##..##..##..##..##..##..##..##..
                ###...###...###...###...###...###...###.
                ####....####....####....####....####....
                #####.....#####.....#####.....#####.....
                ######......######......######......####
                #######.......#######.......#######.....
                "
            )
        );
    }
}
