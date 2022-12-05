use std::fmt::Write as _;

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

type Map = Vec<Vec<String>>;
type Actions = Vec<Action>;

fn parse(input: Vec<String>) -> (Map, Actions) {
    // Split vec in 2 parts (map, actions)
    let mut delim_line = 0;
    for (index, line) in input.iter().enumerate() {
        if line.is_empty() {
            delim_line = index;
        }
    }

    assert!(delim_line != 0);

    let (map, actions) = input.split_at(delim_line);
    dbg!(&map);
    dbg!(&actions);
    let map = parse_map(map);
    let actions = parse_actions(actions);
    (map, actions)
}

fn parse_map(input: &[String]) -> Vec<Vec<String>> {
    // Get number of element
    let mut input = input.to_vec();
    let nb_elem = input
        .pop()
        .unwrap()
        .split("   ")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut map: Vec<Vec<String>> = Vec::new();
    for _ in 0..nb_elem {
        map.push(Vec::new());
    }

    for line in input {
        #[allow(unused_assignments)]
        let (mut case, mut remaining) = line.split_at(0);
        for item in map.iter_mut().take(nb_elem) {
            (case, remaining) = remaining.split_at(3);
            if !remaining.is_empty() {
                remaining = remaining.strip_prefix(' ').unwrap();
            } else {
                remaining = "   ";
            }
            // dbg!(&case);
            // dbg!(&remaining);
            item.push(case.replace(['[', ']'], "").replace("   ", " ").to_string());
        }
    }
    map
}

fn get_map(map: Vec<Vec<String>>) -> String {
    let mut line = Vec::new();
    let height = map.iter().map(|x| x.len()).max().unwrap();
    for case in 0..height {
        let mut column = Vec::new();
        for stack in 0..map.len() {
            column.push(match map.get(stack).unwrap().get(case) {
                Some(x) => match x.as_str() {
                    " " => "   ".to_string(),
                    _ => format!("[{}]", x),
                },

                None => unreachable!(),
            });
        }
        line.push(column)
    }

    let out = line
        .iter()
        .map(|x| x.join(" ").trim_end().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    out
}

#[derive(Debug, PartialEq)]
struct Action {
    nb: u32,
    src: u32,
    dst: u32,
}

fn parse_actions(input: &[String]) -> Vec<Action> {
    let mut input = input.to_vec();
    input.remove(0);

    let actions = input
        .iter_mut()
        .map(|x| {
            x.replace("move", "")
                .replace("from", ",")
                .replace("to", ",")
        })
        .collect::<Vec<String>>()
        .iter()
        .map(|x| {
            let v = x.split(',').collect::<Vec<&str>>();
            Action {
                nb: v[0].trim().parse().unwrap(),
                src: v[1].trim().parse().unwrap(),
                dst: v[2].trim().parse().unwrap(),
            }
        })
        .collect::<Vec<Action>>();

    actions
}

fn run(input: Vec<String>) -> String {
    todo!();
}

fn main() {
    let input = parse_input(None);

    let answer = run(input);

    println!("Player score: {}", answer);
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
    fn test_parse_map_01() {
        let input = indoc!(
            "
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3
            "
        );

        let mut input_lines = parse_input(Some(input));
        let map = parse_map(&input_lines);
        let map = get_map(map);
        println!("{}", map);
        input_lines.pop();
        assert_eq!(input_lines.join("\n"), map);
    }

    #[test]
    fn test_parse_map_02() {
        let input = indoc!(
            "
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3   4
            "
        );

        let mut input_lines = parse_input(Some(input));
        let map = parse_map(&input_lines);
        let map = get_map(map);
        println!("{}", map);
        input_lines.pop();
        assert_eq!(input_lines.join("\n"), map);
    }

    #[test]
    fn test_parse_map_03() {
        let input = indoc!(
            "
                        [D]
                [D]     [C]
            [N] [C]     [B]
            [Z] [M] [P] [A]
             1   2   3   4
            "
        );

        let mut input_lines = parse_input(Some(input));
        let map = parse_map(&input_lines);
        let map = get_map(map);
        println!("{}", map);
        input_lines.pop();
        assert_eq!(input_lines.join("\n"), map);
    }

    #[test]
    fn test_parse_actions_01() {
        let input = indoc!(
            "

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
            "
        );

        let input_lines = parse_input(Some(input));
        let actions = parse_actions(&input_lines);
        assert_eq!(
            actions,
            vec![
                Action {
                    nb: 1,
                    src: 2,
                    dst: 1
                },
                Action {
                    nb: 3,
                    src: 1,
                    dst: 3
                },
                Action {
                    nb: 2,
                    src: 2,
                    dst: 1
                },
                Action {
                    nb: 1,
                    src: 1,
                    dst: 2
                }
            ]
        );
    }

    #[test]
    fn test_parse() {
        let input = parse_input(Some(indoc!(
            "
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
            "
        )));
        let (map, actions) = parse(input);

        assert_eq!(1, 0);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, "CMZ".to_string());
    }
}
