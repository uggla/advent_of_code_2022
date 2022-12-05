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
    let nb_elem = dbg!(input
        .pop()
        .unwrap()
        .split("   ")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap());

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
            if case != "   " {
                item.push(case.replace(['[', ']'], "").to_string());
            }
        }
    }

    for item in map.iter_mut() {
        item.reverse();
    }
    map
}

fn get_map(map: &Vec<Vec<String>>) -> String {
    let mut line = Vec::new();
    let height = map.iter().map(|x| x.len()).max().unwrap();
    for case in 0..height {
        let mut column = Vec::new();
        for stack in 0..map.len() {
            column.push(match map.get(stack).unwrap().get(case) {
                Some(x) => format!("[{}]", x),
                None => "   ".to_string(),
            });
        }
        line.push(column)
    }

    let out = line
        .iter()
        .rev()
        .map(|x| x.join(" ").trim_end().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    out
}

fn top_case(map: Map) -> String {
    let top_case = map
        .iter()
        .map(|x| {
            x.iter()
                .filter(|x| !x.contains(' '))
                .last()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    top_case
}

fn move_case(map: &mut Map, nb: usize, src: usize, dst: usize) {
    let src_stack = map.get(src - 1).unwrap();
    let (lhs, rhs) = src_stack.split_at(src_stack.len() - nb);
    let lhs = lhs.to_vec();
    let mut rhs = rhs.to_vec();
    // rhs.reverse();
    map[src - 1] = lhs;
    map[dst - 1].append(&mut rhs);
}

#[derive(Debug, PartialEq)]
struct Action {
    nb: usize,
    src: usize,
    dst: usize,
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
    let (mut map, actions) = parse(input);
    println!("{}", get_map(&map));
    println!("---------------------------------------------------");
    for action in actions {
        move_case(&mut map, action.nb, action.src, action.dst);
        println!("{}", get_map(&map));
        println!("---------------------------------------------------");
    }
    top_case(map)
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
        println!("{}", get_map(&map));
        input_lines.pop();
        assert_eq!(input_lines.join("\n"), get_map(&map));
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
        println!("{}", get_map(&map));
        input_lines.pop();
        assert_eq!(input_lines.join("\n"), get_map(&map));
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
        println!("{}", get_map(&map));
        input_lines.pop();
        assert_eq!(input_lines.join("\n"), get_map(&map));
    }

    #[test]
    fn test_top_case() {
        let input = indoc!(
            "
                        [D]
                [D]     [C]
            [N] [C]     [B]
            [Z] [M] [P] [A]
             1   2   3   4
            "
        );

        let input_lines = parse_input(Some(input));
        let map = parse_map(&input_lines);
        assert_eq!(top_case(map), "NDPD".to_string());
    }

    #[test]
    fn test_move_case_01() {
        let src = indoc!(
            "
                        [H]
                [D]     [G]
            [N] [C]     [B]
            [Z] [M] [P] [A]
             1   2   3   4
            "
        );
        let dst = indoc!(
            "
                        [D]
                        [C]
                        [H]
                        [G]
            [N]         [B]
            [Z] [M] [P] [A]"
        );

        let input_lines = parse_input(Some(src));
        let mut map = parse_map(&input_lines);
        println!("{}", get_map(&map));
        move_case(&mut map, 2, 2, 4);
        let dst_map = get_map(&map);
        println!("{}", get_map(&map));
        assert_eq!(dst, dst_map)
    }

    #[test]
    fn test_move_case_02() {
        let src = indoc!(
            "
                        [H]
                [D]     [G]
            [N] [C]     [B]
            [Z] [M] [P] [A]
             1   2   3   4
            "
        );
        let dst = indoc!(
            "
                        [H]
                [D]     [G]
                [C] [N] [B]
            [Z] [M] [P] [A]"
        );

        let input_lines = parse_input(Some(src));
        let mut map = parse_map(&input_lines);
        println!("{}", get_map(&map));
        move_case(&mut map, 1, 1, 3);
        let dst_map = get_map(&map);
        println!("{}", get_map(&map));
        assert_eq!(dst, dst_map)
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
        assert_eq!(answer, "MCD".to_string());
    }

    // #[test]
    // fn test_parse_big_map() {
    //     let input = parse_input(Some(indoc!(
    //         "
    //                 [H]         [S]         [D]
    //             [S] [C]         [C]     [Q] [L]
    //             [C] [R] [Z]     [R]     [H] [Z]
    //             [G] [N] [H] [S] [B]     [R] [F]
    //         [D] [T] [Q] [F] [Q] [Z]     [Z] [N]
    //         [Z] [W] [F] [N] [F] [W] [J] [V] [G]
    //         [T] [R] [B] [C] [L] [P] [F] [L] [H]
    //         [H] [Q] [P] [L] [G] [V] [Z] [D] [B]
    //          1   2   3   4   5   6   7   8   9
    //
    //         move 2 from 7 to 2
    //         "
    //     )));
    //     dbg!(&input);
    //     let answer = run(input);
    //     assert_eq!(answer, "DFHZSSZQD".to_string());
    // }
}
