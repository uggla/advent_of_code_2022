use std::collections::HashMap;

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

fn find_first_marker(s: String) -> usize {
    let v = s.chars().enumerate().collect::<Vec<(usize, char)>>();

    v.windows(4)
        .filter_map(|tuple_list| {
            let mut hash: HashMap<char, usize> = HashMap::new();
            tuple_list.iter().for_each(|tuple| {
                let character = hash.entry(tuple.1).or_insert(0);
                *character += 1;
            });
            if hash.len() == 4 {
                Some(tuple_list.last().unwrap().0 + 1)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[allow(dead_code)]
fn find_all_markers(s: String) -> Vec<usize> {
    let v = s.chars().enumerate().collect::<Vec<(usize, char)>>();

    v.windows(4)
        .filter_map(|tuple_list| {
            let mut hash: HashMap<char, usize> = HashMap::new();
            tuple_list.iter().for_each(|tuple| {
                let character = hash.entry(tuple.1).or_insert(0);
                *character += 1;
            });
            if hash.len() == 4 {
                Some(tuple_list.last().unwrap().0 + 1)
            } else {
                None
            }
        })
        .collect()
}

fn run(input: Vec<String>) -> usize {
    find_first_marker(input.join(""))
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
    fn test_find_first_marker_01() {
        let s = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(find_first_marker(s), 7);
    }

    #[test]
    fn test_find_first_marker_02() {
        let s = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(find_first_marker(s), 5);
    }

    #[test]
    fn test_find_first_marker_03() {
        let s = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(find_first_marker(s), 6);
    }

    #[test]
    fn test_find_first_marker_04() {
        let s = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(find_first_marker(s), 10);
    }

    #[test]
    fn test_find_first_marker_05() {
        let s = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(find_first_marker(s), 11);
    }

    #[test]
    fn test_find_all_markers_01() {
        let s = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        dbg!(find_all_markers(s));
        // assert_eq!(10, 11);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 11);
    }
}
