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
enum Visible {
    Yes,
    No,
}

type Map = Vec<Vec<isize>>;
type Y = isize;
type X = isize;

#[derive(Debug, Clone)]
struct Coord(Y, X);

impl Coord {
    fn new(y: Y, x: X) -> Self {
        Self(y, x)
    }

    fn up(&mut self) {
        self.0 -= 1;
    }

    fn down(&mut self) {
        self.0 += 1;
    }

    fn left(&mut self) {
        self.1 -= 1;
    }

    fn right(&mut self) {
        self.1 += 1;
    }
}

fn parse_map(input: Vec<String>) -> Map {
    let mut map = Vec::new();
    for line in input {
        // let v = line.chars
        let v: Vec<isize> = line
            .chars()
            .map(|o| o.to_string().parse().unwrap())
            .collect();

        map.push(v);
    }
    map
}

fn check_visible_up(pos: &Coord, map: &Map) -> Visible {
    let mut pos = pos.clone();
    let tree_size = |pos: &Coord| map[pos.0 as usize][pos.1 as usize];

    let origin_tree_size = tree_size(&pos);
    // Going up
    while pos.0 > 0 {
        pos.up();
        if tree_size(&pos) >= origin_tree_size {
            return Visible::No;
        }
    }
    Visible::Yes
}

fn check_visible_down(pos: &Coord, map: &Map) -> Visible {
    let mut pos = pos.clone();
    let y_max: isize = isize::try_from(map.len()).unwrap() - 1;

    let tree_size = |pos: &Coord| map[pos.0 as usize][pos.1 as usize];

    let origin_tree_size = tree_size(&pos);
    // Going down
    while pos.0 < y_max {
        pos.down();
        if tree_size(&pos) >= origin_tree_size {
            return Visible::No;
        }
    }
    Visible::Yes
}

fn check_visible_left(pos: &Coord, map: &Map) -> Visible {
    let mut pos = pos.clone();
    let tree_size = |pos: &Coord| map[pos.0 as usize][pos.1 as usize];

    let origin_tree_size = tree_size(&pos);
    // Going left
    while pos.1 > 0 {
        pos.left();
        if tree_size(&pos) >= origin_tree_size {
            return Visible::No;
        }
    }
    Visible::Yes
}

fn check_visible_right(pos: &Coord, map: &Map) -> Visible {
    let mut pos = pos.clone();
    let x_max: isize = isize::try_from(map[0].len()).unwrap() - 1;

    let tree_size = |pos: &Coord| map[pos.0 as usize][pos.1 as usize];

    let origin_tree_size = tree_size(&pos);
    // Going right
    while pos.1 < x_max {
        pos.right();
        if tree_size(&pos) >= origin_tree_size {
            return Visible::No;
        }
    }
    Visible::Yes
}

fn check_visible(pos: &Coord, map: &Map) -> Visible {
    match (
        check_visible_up(pos, map),
        check_visible_down(pos, map),
        check_visible_left(pos, map),
        check_visible_right(pos, map),
    ) {
        (Visible::No, Visible::No, Visible::No, Visible::No) => Visible::No,
        _ => Visible::Yes,
    }
}

fn run(input: Vec<String>) -> usize {
    let map = parse_map(input);
    let mut nb_visible: usize = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if check_visible(
                &Coord::new(y.try_into().unwrap(), x.try_into().unwrap()),
                &map,
            ) == Visible::Yes
            {
                nb_visible += 1;
            }
        }
    }
    nb_visible
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
    fn test_visible_directions() {
        let input = parse_input(Some(indoc!(
            "
            30373
            25512
            65332
            33549
            35390
            "
        )));
        let map = parse_map(input);
        assert_eq!(check_visible_up(&Coord::new(0, 0), &map), Visible::Yes);
        assert_eq!(check_visible_up(&Coord::new(1, 0), &map), Visible::No);
        assert_eq!(check_visible_up(&Coord::new(2, 0), &map), Visible::Yes);
        assert_eq!(check_visible_down(&Coord::new(0, 0), &map), Visible::No);
        assert_eq!(check_visible_down(&Coord::new(1, 0), &map), Visible::No);
        assert_eq!(check_visible_down(&Coord::new(2, 0), &map), Visible::Yes);
        assert_eq!(check_visible_left(&Coord::new(0, 0), &map), Visible::Yes);
        assert_eq!(check_visible_left(&Coord::new(1, 0), &map), Visible::Yes);
        assert_eq!(check_visible_left(&Coord::new(2, 0), &map), Visible::Yes);
        assert_eq!(check_visible_right(&Coord::new(0, 0), &map), Visible::No);
        assert_eq!(check_visible_right(&Coord::new(1, 0), &map), Visible::No);
        assert_eq!(check_visible_right(&Coord::new(2, 0), &map), Visible::Yes);

        assert_eq!(check_visible_up(&Coord::new(1, 1), &map), Visible::Yes);
        assert_eq!(check_visible_down(&Coord::new(1, 1), &map), Visible::No);
        assert_eq!(check_visible_left(&Coord::new(1, 1), &map), Visible::Yes);
        assert_eq!(check_visible_right(&Coord::new(1, 1), &map), Visible::No);

        assert_eq!(check_visible_up(&Coord::new(1, 2), &map), Visible::Yes);
        assert_eq!(check_visible_down(&Coord::new(1, 2), &map), Visible::No);
        assert_eq!(check_visible_left(&Coord::new(1, 2), &map), Visible::No);
        assert_eq!(check_visible_right(&Coord::new(1, 2), &map), Visible::Yes);

        assert_eq!(check_visible_up(&Coord::new(1, 3), &map), Visible::No);
        assert_eq!(check_visible_down(&Coord::new(1, 3), &map), Visible::No);
        assert_eq!(check_visible_left(&Coord::new(1, 3), &map), Visible::No);
        assert_eq!(check_visible_right(&Coord::new(1, 3), &map), Visible::No);
    }

    #[test]
    fn test_visible() {
        let input = parse_input(Some(indoc!(
            "
            30373
            25512
            65332
            33549
            35390
            "
        )));
        let map = parse_map(input);
        assert_eq!(check_visible(&Coord::new(1, 3), &map), Visible::No);
        assert_eq!(check_visible(&Coord::new(3, 2), &map), Visible::Yes);
    }

    #[test]
    fn test_parse() {
        let input = parse_input(Some(indoc!(
            "
            30373
            25512
            65332
            33549
            35390
            "
        )));
        let map = parse_map(input);
        dbg!(&map);
        assert_eq!(map[1][2], 5);
        assert_eq!(map[4][4], 0);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            30373
            25512
            65332
            33549
            35390
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 21);
    }
}
