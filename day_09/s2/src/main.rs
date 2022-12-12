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
    visited: HashSet<(X, Y)>,
}

impl Location {
    fn new(x: X, y: Y) -> Self {
        let mut set = HashSet::new();
        set.insert((0, 0));
        Self {
            x,
            y,
            nb_move: 0,
            visited: set,
        }
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

fn shift(rope: &mut [Location], direction: &Move) {
    const LENGTH: u32 = 1;
    match direction {
        Move::Up(t2) => {
            for _ in 0..*t2 {
                rope[0].up();

                for i in 1..rope.len() {
                    if distance(&rope[i - 1], &rope[i]) > LENGTH.try_into().unwrap() {
                        if rope[i - 1].y < rope[i].y {
                            rope[i].down();
                        }
                        if rope[i - 1].y > rope[i].y {
                            rope[i].up();
                        }
                        if rope[i - 1].x < rope[i].x {
                            rope[i].left();
                        }
                        if rope[i - 1].x > rope[i].x {
                            rope[i].right();
                        }
                        rope[i].visited.insert((rope[i].x, rope[i].y));
                    }
                }
            }
        }
        Move::Down(t2) => {
            for _ in 0..*t2 {
                rope[0].down();

                for i in 1..rope.len() {
                    if distance(&rope[i - 1], &rope[i]) > LENGTH.try_into().unwrap() {
                        if rope[i - 1].y < rope[i].y {
                            rope[i].down();
                        }
                        if rope[i - 1].y > rope[i].y {
                            rope[i].up();
                        }
                        if rope[i - 1].x < rope[i].x {
                            rope[i].left();
                        }
                        if rope[i - 1].x > rope[i].x {
                            rope[i].right();
                        }
                        rope[i].visited.insert((rope[i].x, rope[i].y));
                    }
                }
            }
        }
        Move::Right(t2) => {
            for _ in 0..*t2 {
                rope[0].right();

                for i in 1..rope.len() {
                    if distance(&rope[i - 1], &rope[i]) > LENGTH.try_into().unwrap() {
                        if rope[i - 1].y < rope[i].y {
                            rope[i].down();
                        }
                        if rope[i - 1].y > rope[i].y {
                            rope[i].up();
                        }
                        if rope[i - 1].x < rope[i].x {
                            rope[i].left();
                        }
                        if rope[i - 1].x > rope[i].x {
                            rope[i].right();
                        }
                        rope[i].visited.insert((rope[i].x, rope[i].y));
                    }
                }
            }
        }
        Move::Left(t2) => {
            for _ in 0..*t2 {
                rope[0].left();

                for i in 1..rope.len() {
                    if distance(&rope[i - 1], &rope[i]) > LENGTH.try_into().unwrap() {
                        if rope[i - 1].y < rope[i].y {
                            rope[i].down();
                        }
                        if rope[i - 1].y > rope[i].y {
                            rope[i].up();
                        }
                        if rope[i - 1].x < rope[i].x {
                            rope[i].left();
                        }
                        if rope[i - 1].x > rope[i].x {
                            rope[i].right();
                        }
                        rope[i].visited.insert((rope[i].x, rope[i].y));
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
    let mut rope = Vec::new();
    let mut tail_visited = HashSet::new();
    tail_visited.insert((0, 0));

    for _ in 0..10 {
        rope.push(Location::new(0, 0))
    }

    for mov in &moves {
        shift(&mut rope, mov);
    }
    for i in &rope {
        dbg!(&i.x, &i.y, "---");
    }
    // dbg!(&rope.last());
    rope.last().unwrap().visited.len()
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
        shift(&mut rope, &Move::Right(3));
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 2isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Up(4));
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 4isize);
        assert_eq!(rope[1].x, 3isize);
        assert_eq!(rope[1].y, 3isize);

        shift(&mut rope, &Move::Left(3));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 4isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 4isize);

        shift(&mut rope, &Move::Down(4));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Up(5));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 4isize);

        shift(&mut rope, &Move::Right(5));
        assert_eq!(rope[0].x, 5isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 4isize);
        assert_eq!(rope[1].y, 5isize);

        shift(&mut rope, &Move::Down(5));
        assert_eq!(rope[0].x, 5isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Left(5));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Right(5));
        assert_eq!(rope[0].x, 5isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 4isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Left(5));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 0isize);

        shift(&mut rope, &Move::Up(9));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 9isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 8isize);

        shift(&mut rope, &Move::Down(9));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Up(2));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        shift(&mut rope, &Move::Right(2));
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Left(2));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Up(1));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Right(1));
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Right(1));
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Down(1));
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Down(1));
        assert_eq!(rope[0].x, 2isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Left(1));
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Left(1));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Up(1));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 2isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 2isize);

        shift(&mut rope, &Move::Down(2));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 0isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(1, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Up(2));
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 4isize);

        let mut rope = vec![Location::new(1, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Up(2));
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 5isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 4isize);

        let mut rope = vec![Location::new(1, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Down(3));
        assert_eq!(rope[0].x, 1isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3));
        assert_eq!(rope[0].x, 6isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3));
        assert_eq!(rope[0].x, 6isize);
        assert_eq!(rope[0].y, 1isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Up(3));
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 4isize);
        assert_eq!(rope[1].x, 3isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 1), Location::new(2, 2)];
        shift(&mut rope, &Move::Down(1));
        assert_eq!(rope[0].x, 3isize);
        assert_eq!(rope[0].y, 0isize);
        assert_eq!(rope[1].x, 3isize);
        assert_eq!(rope[1].y, 1isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3));
        assert_eq!(rope[0].x, 6isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 5isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Left(3));
        assert_eq!(rope[0].x, 0isize);
        assert_eq!(rope[0].y, 3isize);
        assert_eq!(rope[1].x, 1isize);
        assert_eq!(rope[1].y, 3isize);

        let mut rope = vec![Location::new(3, 3), Location::new(2, 2)];
        shift(&mut rope, &Move::Right(3));
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
    fn test_simple_move() {
        let input = parse_input(Some(indoc!(
            "
            R 5
            U 8
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 1);
        // assert_eq!(true, false);
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
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_run2() {
        let input = parse_input(Some(indoc!(
            "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 36);
    }
}
