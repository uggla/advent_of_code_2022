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

fn shift_refactor(
    head: &mut Location,
    tail: &mut Location,
    direction: &Move,
    tail_visited: &mut HashSet<(X, Y)>,
) {
    const LENGTH: u32 = 1;
    match direction {
        Move::Up(t2) => {
            for _ in 0..*t2 {
                head.up();
                if distance(head, tail) > LENGTH.try_into().unwrap() {
                    if head.x < tail.x {
                        tail.left();
                    }
                    if head.x > tail.x {
                        tail.right();
                    }
                    tail.up();
                    tail_visited.insert((tail.x, tail.y));
                }
            }
        }
        Move::Down(t2) => {
            for _ in 0..*t2 {
                head.down();
                if distance(head, tail) > LENGTH.try_into().unwrap() {
                    if head.x < tail.x {
                        tail.left();
                    }
                    if head.x > tail.x {
                        tail.right();
                    }
                    tail.down();
                    tail_visited.insert((tail.x, tail.y));
                }
            }
        }
        Move::Right(t2) => {
            for _ in 0..*t2 {
                head.right();
                if distance(head, tail) > LENGTH.try_into().unwrap() {
                    if head.y < tail.y {
                        tail.down();
                    }
                    if head.y > tail.y {
                        tail.up();
                    }
                    tail.right();
                    tail_visited.insert((tail.x, tail.y));
                }
            }
        }
        Move::Left(t2) => {
            for _ in 0..*t2 {
                head.left();
                if distance(head, tail) > LENGTH.try_into().unwrap() {
                    if head.y < tail.y {
                        tail.down();
                    }
                    if head.y > tail.y {
                        tail.up();
                    }
                    tail.left();
                    tail_visited.insert((tail.x, tail.y));
                }
            }
        }
    }
}

fn shift(
    head: &mut Location,
    tail: &mut Location,
    direction: &Move,
    prev_direction: &mut Move,
    tail_visited: &mut HashSet<(X, Y)>,
) {
    shift_refactor(head, tail, direction, tail_visited)
    // const LENGTH: u32 = 1;
    // match (prev_direction, direction) {
    //     (Move::Up(_), Move::Up(t2)) => {
    //         for _ in 0..*t2 {
    //             head.up();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.x < tail.x {
    //                     tail.left();
    //                 }
    //                 if head.x > tail.x {
    //                     tail.right();
    //                 }
    //                 tail.up();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Up(_), Move::Down(t2)) => {
    //         for _ in 0..*t2 {
    //             head.down();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.x < tail.x {
    //                     tail.left();
    //                 }
    //                 if head.x > tail.x {
    //                     tail.right();
    //                 }
    //                 tail.down();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Up(_), Move::Left(t2)) => {
    //         for _ in 0..*t2 {
    //             head.left();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y != tail.y {
    //                     tail.up();
    //                 }
    //                 tail.left();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Up(_), Move::Right(t2)) => {
    //         for _ in 0..*t2 {
    //             head.right();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y != tail.y {
    //                     tail.up();
    //                 }
    //                 tail.right();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Down(_), Move::Up(t2)) => {
    //         for _ in 0..*t2 {
    //             head.up();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.x < tail.x {
    //                     tail.left();
    //                 }
    //                 if head.x > tail.x {
    //                     tail.right();
    //                 }
    //                 tail.up();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Down(_), Move::Down(t2)) => {
    //         for _ in 0..*t2 {
    //             head.down();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.x < tail.x {
    //                     tail.left();
    //                 }
    //                 if head.x > tail.x {
    //                     tail.right();
    //                 }
    //                 tail.down();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Down(_), Move::Left(t2)) => {
    //         for _ in 0..*t2 {
    //             head.left();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y != tail.y {
    //                     tail.down();
    //                 }
    //                 tail.left();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Down(_), Move::Right(t2)) => {
    //         for _ in 0..*t2 {
    //             head.right();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y != tail.y {
    //                     tail.down();
    //                 }
    //                 tail.right();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Left(_), Move::Up(t2)) => {
    //         for _ in 0..*t2 {
    //             head.up();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.x != tail.x {
    //                     tail.left();
    //                 }
    //                 tail.up();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Left(_), Move::Down(t2)) => {
    //         for _ in 0..*t2 {
    //             head.down();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.x != tail.x {
    //                     tail.left();
    //                 }
    //                 tail.down();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Left(_), Move::Left(t2)) => {
    //         for _ in 0..*t2 {
    //             head.left();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y < tail.y {
    //                     tail.down();
    //                 }
    //                 if head.y > tail.y {
    //                     tail.up();
    //                 }
    //                 tail.left();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Left(_), Move::Right(t2)) => {
    //         for _ in 0..*t2 {
    //             head.right();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y < tail.y {
    //                     tail.down();
    //                 }
    //                 if head.y > tail.y {
    //                     tail.up();
    //                 }
    //                 tail.right();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Right(_), Move::Up(t2)) => {
    //         for _ in 0..*t2 {
    //             head.up();
    //             if dbg!(distance(head, tail)) > LENGTH.try_into().unwrap() {
    //                 if head.x != tail.x {
    //                     tail.right();
    //                 }
    //                 tail.up();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Right(_), Move::Down(t2)) => {
    //         for _ in 0..*t2 {
    //             head.down();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.x != tail.x {
    //                     tail.right();
    //                 }
    //                 tail.down();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Right(_), Move::Left(t2)) => {
    //         for _ in 0..*t2 {
    //             head.left();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y < tail.y {
    //                     tail.down();
    //                 }
    //                 if head.y > tail.y {
    //                     tail.up();
    //                 }
    //                 tail.left();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    //     (Move::Right(_), Move::Right(t2)) => {
    //         for _ in 0..*t2 {
    //             head.right();
    //             if distance(head, tail) > LENGTH.try_into().unwrap() {
    //                 if head.y < tail.y {
    //                     tail.down();
    //                 }
    //                 if head.y > tail.y {
    //                     tail.up();
    //                 }
    //                 tail.right();
    //                 tail_visited.insert((tail.x, tail.y));
    //             }
    //         }
    //     }
    // }
}

fn distance(item1: &Location, item2: &Location) -> isize {
    let dist = (((item2.x - item1.x).pow(2) + (item2.y - item1.y).pow(2)) as f32).sqrt();
    dist as isize
}

fn run(input: Vec<String>) -> usize {
    let moves = parse_line(input);
    dbg!(&moves);
    let mut head = Location::new(0, 0);
    let mut tail = Location::new(0, 0);
    let mut tail_visited = HashSet::new();
    tail_visited.insert((0, 0));
    let mut previous_move = Move::Up(0);
    for mov in &moves {
        shift(
            &mut head,
            &mut tail,
            mov,
            &mut previous_move,
            &mut tail_visited,
        );
        previous_move = mov.clone();
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
        let mut head = Location::new(0, 0);
        let mut tail = Location::new(0, 0);
        let mut tail_visited = HashSet::new();
        shift(
            &mut head,
            &mut tail,
            &Move::Right(3),
            &mut Move::Up(0),
            &mut tail_visited,
        );
        assert_eq!(head.x, 3isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 2isize);
        assert_eq!(tail.y, 0isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Up(4),
            &mut Move::Right(3),
            &mut tail_visited,
        );
        assert_eq!(head.x, 3isize);
        assert_eq!(head.y, 4isize);
        assert_eq!(tail.x, 3isize);
        assert_eq!(tail.y, 3isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Left(3),
            &mut Move::Up(4),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 4isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 4isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Down(4),
            &mut Move::Left(3),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 0isize);
        assert_eq!(tail.y, 1isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Up(5),
            &mut Move::Down(4),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 5isize);
        assert_eq!(tail.x, 0isize);
        assert_eq!(tail.y, 4isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Right(5),
            &mut Move::Up(5),
            &mut tail_visited,
        );
        assert_eq!(head.x, 5isize);
        assert_eq!(head.y, 5isize);
        assert_eq!(tail.x, 4isize);
        assert_eq!(tail.y, 5isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Down(5),
            &mut Move::Right(5),
            &mut tail_visited,
        );
        assert_eq!(head.x, 5isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 5isize);
        assert_eq!(tail.y, 1isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Left(5),
            &mut Move::Down(5),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 0isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Right(5),
            &mut Move::Left(5),
            &mut tail_visited,
        );
        assert_eq!(head.x, 5isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 4isize);
        assert_eq!(tail.y, 0isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Left(5),
            &mut Move::Right(5),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 0isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Up(9),
            &mut Move::Left(5),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 9isize);
        assert_eq!(tail.x, 0isize);
        assert_eq!(tail.y, 8isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Down(9),
            &mut Move::Up(9),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 0isize);
        assert_eq!(tail.y, 1isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Up(2),
            &mut Move::Down(9),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 2isize);
        assert_eq!(tail.x, 0isize);
        assert_eq!(tail.y, 1isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Right(2),
            &mut Move::Up(2),
            &mut tail_visited,
        );
        assert_eq!(head.x, 2isize);
        assert_eq!(head.y, 2isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Left(2),
            &mut Move::Right(2),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 2isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Up(1),
            &mut Move::Left(2),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 3isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Right(1),
            &mut Move::Up(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 1isize);
        assert_eq!(head.y, 3isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Right(1),
            &mut Move::Right(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 2isize);
        assert_eq!(head.y, 3isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Down(1),
            &mut Move::Right(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 2isize);
        assert_eq!(head.y, 2isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Down(1),
            &mut Move::Down(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 2isize);
        assert_eq!(head.y, 1isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Left(1),
            &mut Move::Down(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 1isize);
        assert_eq!(head.y, 1isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Left(1),
            &mut Move::Left(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 1isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Up(1),
            &mut Move::Left(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 2isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 2isize);

        shift(
            &mut head,
            &mut tail,
            &Move::Down(2),
            &mut Move::Up(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 0isize);
        assert_eq!(tail.y, 1isize);

        head = Location::new(1, 3);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Up(2),
            &mut Move::Up(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 1isize);
        assert_eq!(head.y, 5isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 4isize);

        head = Location::new(1, 3);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Up(2),
            &mut Move::Down(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 1isize);
        assert_eq!(head.y, 5isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 4isize);

        head = Location::new(1, 3);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Down(3),
            &mut Move::Down(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 1isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 1isize);

        head = Location::new(3, 1);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Left(3),
            &mut Move::Left(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 1isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 1isize);

        head = Location::new(3, 1);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Right(3),
            &mut Move::Left(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 6isize);
        assert_eq!(head.y, 1isize);
        assert_eq!(tail.x, 5isize);
        assert_eq!(tail.y, 1isize);

        head = Location::new(3, 1);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Left(3),
            &mut Move::Right(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 1isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 1isize);

        head = Location::new(3, 1);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Right(3),
            &mut Move::Right(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 6isize);
        assert_eq!(head.y, 1isize);
        assert_eq!(tail.x, 5isize);
        assert_eq!(tail.y, 1isize);

        head = Location::new(3, 1);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Up(3),
            &mut Move::Up(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 3isize);
        assert_eq!(head.y, 4isize);
        assert_eq!(tail.x, 3isize);
        assert_eq!(tail.y, 3isize);

        head = Location::new(3, 1);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Down(1),
            &mut Move::Up(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 3isize);
        assert_eq!(head.y, 0isize);
        assert_eq!(tail.x, 3isize);
        assert_eq!(tail.y, 1isize);

        head = Location::new(3, 3);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Left(3),
            &mut Move::Left(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 3isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 3isize);

        head = Location::new(3, 3);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Right(3),
            &mut Move::Left(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 6isize);
        assert_eq!(head.y, 3isize);
        assert_eq!(tail.x, 5isize);
        assert_eq!(tail.y, 3isize);

        head = Location::new(3, 3);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Left(3),
            &mut Move::Right(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 0isize);
        assert_eq!(head.y, 3isize);
        assert_eq!(tail.x, 1isize);
        assert_eq!(tail.y, 3isize);

        head = Location::new(3, 3);
        tail = Location::new(2, 2);
        shift(
            &mut head,
            &mut tail,
            &Move::Right(3),
            &mut Move::Right(1),
            &mut tail_visited,
        );
        assert_eq!(head.x, 6isize);
        assert_eq!(head.y, 3isize);
        assert_eq!(tail.x, 5isize);
        assert_eq!(tail.y, 3isize);
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
