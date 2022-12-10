use std::collections::HashSet;

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

type Y = isize;
type X = isize;

#[derive(Debug, Clone)]
struct Location {
    x: isize,
    y: isize,
    nb_move: usize,
}

impl Location {
    fn new(x: X, y: Y) -> Self {
        Self { x, y, nb_move: 0 }
    }

    fn up(&mut self) {
        self.y += 1;
        self.nb_move += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
        self.nb_move += 1;
    }

    fn left(&mut self) {
        self.x -= 1;
        self.nb_move += 1;
    }

    fn right(&mut self) {
        self.x += 1;
        self.nb_move += 1;
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

fn parse_line(input: Vec<String>) -> Vec<Move> {
    let moves = input
        .iter()
        .map(|line| {
            let v = line.split(' ').collect::<Vec<&str>>();
            match v[0] {
                "U" => Move::Up(v[1].parse().unwrap()),
                "D" => Move::Down(v[1].parse().unwrap()),
                "L" => Move::Left(v[1].parse().unwrap()),
                "R" => Move::Right(v[1].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Move>>();
    moves
}

fn shift(rope: &mut [Location], direction: &Move, tail_visited: &mut HashSet<(X, Y)>) {
    const LENGTH: u32 = 1;
    match direction {
        Move::Up(t2) => {
            for _ in 0..*t2 {
                for nodes in rope.chunks_mut(2) {
                    nodes[0].up();
                    if distance(&nodes[0], &nodes[1]) > LENGTH.try_into().unwrap() {
                        if nodes[0].x < nodes[1].x {
                            nodes[1].left();
                        }
                        if nodes[0].x > nodes[1].x {
                            nodes[1].right();
                        }
                        nodes[1].up();
                        tail_visited.insert((nodes[1].x, nodes[1].y));
                    }
                }
            }
        }
        Move::Down(t2) => {
            for _ in 0..*t2 {
                for nodes in rope.chunks_mut(2) {
                    nodes[0].down();
                    if distance(&nodes[0], &nodes[1]) > LENGTH.try_into().unwrap() {
                        if nodes[0].x < nodes[1].x {
                            nodes[1].left();
                        }
                        if nodes[0].x > nodes[1].x {
                            nodes[1].right();
                        }
                        nodes[1].down();
                        tail_visited.insert((nodes[1].x, nodes[1].y));
                    }
                }
            }
        }
        Move::Right(t2) => {
            for _ in 0..*t2 {
                for nodes in rope.chunks_mut(2) {
                    nodes[0].right();
                    if distance(&nodes[0], &nodes[1]) > LENGTH.try_into().unwrap() {
                        if nodes[0].y < nodes[1].y {
                            nodes[1].down();
                        }
                        if nodes[0].y > nodes[1].y {
                            nodes[1].up();
                        }
                        nodes[1].right();
                        tail_visited.insert((nodes[1].x, nodes[1].y));
                    }
                }
            }
        }
        Move::Left(t2) => {
            for _ in 0..*t2 {
                for nodes in rope.chunks_mut(2) {
                    nodes[0].left();
                    if distance(&nodes[0], &nodes[1]) > LENGTH.try_into().unwrap() {
                        if nodes[0].y < nodes[1].y {
                            nodes[1].down();
                        }
                        if nodes[0].y > nodes[1].y {
                            nodes[1].up();
                        }
                        nodes[1].left();
                        tail_visited.insert((nodes[1].x, nodes[1].y));
                    }
                }
            }
        }
    }
}

fn distance(item1: &Location, item2: &Location) -> isize {
    let dist = (((item2.x - item1.x).pow(2) + (item2.y - item1.y).pow(2)) as f32).sqrt();
    dist as isize
}

fn run(input: Vec<String>) -> usize {
    let moves = parse_line(input);
    dbg!(&moves);
    let mut rope = vec![Location::new(0, 0), Location::new(0, 0)];
    let mut tail_visited = HashSet::new();
    tail_visited.insert((0, 0));
    for mov in &moves {
        shift(&mut rope, mov, &mut tail_visited);
    }
    dbg!(&tail_visited);
    tail_visited.len()
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
    fn test_shift() {
        let mut rope = vec![Location::new(0, 0), Location::new(0, 0)];
        let mut tail_visited = HashSet::new();
        shift(&mut rope, &Move::Right(3), &mut tail_visited);
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 2isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Up(4), &mut tail_visited);
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 4isize);
        assert_eq!(rope[1].x, 3isize);
        assert_eq!(rope[1].y, 3isize);

        shift(&mut rope, &Move::Left(3), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 4isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 4isize);

        shift(&mut rope, &Move::Down(4), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Up(5), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 4isize);

        shift(&mut rope, &Move::Right(5), &mut tail_visited);
        assert_eq!(rope[0].x, 5isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 4isize);
        assert_eq!(rope[1].y, 5isize);

        shift(&mut rope, &Move::Down(5), &mut tail_visited);
        assert_eq!(rope[0].x, 5isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Left(5), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Right(5), &mut tail_visited);
        assert_eq!(rope[0].x, 5isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 4isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Left(5), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Up(9), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 9isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 8isize);

        shift(&mut rope, &Move::Down(9), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Up(2), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Right(2), &mut tail_visited);
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Left(2), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Up(1), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Right(1), &mut tail_visited);
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Right(1), &mut tail_visited);
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Down(1), &mut tail_visited);
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Down(1), &mut tail_visited);
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Left(1), &mut tail_visited);
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Left(1), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Up(1), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Down(2), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(1, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Up(2), &mut tail_visited);
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 4isize);

        let mut rope = vec![Location::new(1, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Up(2), &mut tail_visited);
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 4isize);

        let mut rope = vec![Location::new(1, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Down(3), &mut tail_visited);
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3), &mut tail_visited);
        assert_eq!(rope[0].x, 6isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3), &mut tail_visited);
        assert_eq!(rope[0].x, 6isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Up(3), &mut tail_visited);
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 4isize);
        assert_eq!(rope[1].x, 3isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Down(1), &mut tail_visited);
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 3isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3), &mut tail_visited);
        assert_eq!(rope[0].x, 6isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3), &mut tail_visited);
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3), &mut tail_visited);
        assert_eq!(rope[0].x, 6isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 3isize);
    }

    #[test]
    fn test_distance() {
        let head = Location::new(0, 0);
        let tail = Location::new(0, 0);
        assert_eq!(distance(&head, &tail), 0);
        let head = Location::new(3, 0);
        let tail = Location::new(0, 0);
        assert_eq!(distance(&head, &tail), 3);
        let head = Location::new(1, 1);
        let tail = Location::new(0, 0);
        assert_eq!(distance(&head, &tail), 1);
        let head = Location::new(2, 2);
        let tail = Location::new(1, 1);
        assert_eq!(distance(&head, &tail), 1);
        let head = Location::new(3, 1);
        let tail = Location::new(1, 1);
        assert_eq!(distance(&head, &tail), 2);
        let head = Location::new(3, 2);
        let tail = Location::new(1, 1);
        assert_eq!(distance(&head, &tail), 2);
        let head = Location::new(3, 3);
        let tail = Location::new(1, 1);
        assert_eq!(distance(&head, &tail), 2);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 13);
    }
}
