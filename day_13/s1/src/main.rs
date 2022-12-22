use std::cmp::Ordering;

use serde::Deserialize;

fn parse_input(input: Option<&str>) -> Vec<String> {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };
    let output = input
        .strip_suffix('\n')
        .unwrap()
        .split("\n\n")
        .map(|o| o.to_string())
        .collect::<Vec<String>>();

    output
}

#[derive(Deserialize, Clone, Eq, Debug)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::Number(r0)) => l0 == &vec![Node::Number(*r0)],
            (Self::Number(l0), Self::List(r0)) => &vec![Node::Number(*l0)] == r0,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Node::List(a), Node::List(b)) => a.cmp(b),
            (Node::List(a), Node::Number(b)) => a.cmp(&vec![Node::Number(*b)]),
            (Node::Number(a), Node::List(b)) => vec![Node::Number(*a)].cmp(b),
            (Node::Number(a), Node::Number(b)) => a.cmp(b),
        }
    }
}

fn run(input: Vec<String>) -> usize {
    // let mut nodes = input.iter().map(|line| serde_json(line))
    let mut groups = Vec::new();
    let mut res: Vec<usize> = Vec::new();
    for group in input {
        let items = group
            .split('\n')
            .map(|o| serde_json::from_str::<Node>(o).unwrap())
            .collect::<Vec<Node>>();
        groups.push(items);
    }
    dbg!(&groups);

    for (index, group) in groups.iter().enumerate() {
        match group[0].cmp(&group[1]) {
            Ordering::Less => {
                println!("{:?} < {:?} --> Ok", group[0], group[1]);
                res.push(index);
            }
            Ordering::Equal => unreachable!(),
            Ordering::Greater => {
                println!("{:?} < {:?} --> Ko", group[0], group[1]);
            }
        }
    }

    res.iter_mut().for_each(|o| *o += 1);
    dbg!(&res);

    res.iter().sum()
}

fn main() {
    let input = parse_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
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
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 13);
    }
}
