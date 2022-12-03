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

fn build_index() -> HashMap<String, usize> {
    let mut index: HashMap<String, usize> = HashMap::new();
    for (id, item) in ('a'..='z').enumerate() {
        index.insert(item.to_string(), id + 1);
    }

    for (id, item) in ('A'..='Z').enumerate() {
        index.insert(item.to_string(), id + 27);
    }

    index
}

fn find_common_char(s1: &str, s2: &str) -> Option<String> {
    for character in s1.chars() {
        if s2.contains(character) {
            return Some(character.to_string());
        }
    }
    None
}

fn run(input: Vec<String>) -> u32 {
    let mut output = 0;
    let index = build_index();
    for line in input {
        let (first_part, last_part) = line.split_at(line.len() / 2);
        let common_char = find_common_char(first_part, last_part).unwrap();
        output += index.get(&common_char).unwrap();
    }
    output.try_into().unwrap()
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
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 157);
    }
}
