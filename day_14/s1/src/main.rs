use core::fmt;
use core::fmt::Debug;
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

struct Map {
    height: usize,
    width: usize,
    data: Vec<Cell>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::new();
        for _ in 0..height {
            for _ in 0..width {
                data.push(Cell::Air);
            }
        }
        Self {
            height,
            width,
            data,
        }
    }

    fn bound(&self, coord: &Coord) -> bool {
        coord.x < self.width as isize
            && coord.y < self.height as isize
            && coord.x >= 0
            && coord.y >= 0
    }

    fn get_cell(&self, coord: Coord) -> Option<&Cell> {
        if !self.bound(&coord) {
            return None;
        }
        Some(&self.data[coord.y as usize * self.width + coord.x as usize])
    }

    fn get_mut_cell(&mut self, coord: Coord) -> Option<&mut Cell> {
        if !self.bound(&coord) {
            return None;
        }
        Some(&mut self.data[coord.y as usize * self.width + coord.x as usize])
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}x{} grid:", self.width, self.height)?;
        for y in 0..self.height {
            let mut line = Vec::new();
            for x in 0..self.width {
                let cell = self
                    .get_cell((x.try_into().unwrap(), y.try_into().unwrap()).into())
                    .unwrap();
                let c = match cell {
                    Cell::Rock => '#',
                    Cell::Air => '.',
                    Cell::Sand => 'o',
                };
                line.push(c.to_string());
            }
            write!(f, "{}", line.join(""))?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Air,
    Sand,
    Rock,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn check_down(&self, map: &Map) -> bool {
        match map.get_cell((self.x, self.y + 1).into()) {
            None => false,
            Some(cell) => match cell {
                Cell::Rock => false,
                Cell::Sand => false,
                Cell::Air => true,
            },
        }
    }

    fn check_left(&self, map: &Map) -> bool {
        match map.get_cell((self.x - 1, self.y + 1).into()) {
            None => false,
            Some(cell) => match cell {
                Cell::Rock => false,
                Cell::Sand => false,
                Cell::Air => true,
            },
        }
    }

    fn check_right(&self, map: &Map) -> bool {
        match map.get_cell((self.x + 1, self.y + 1).into()) {
            None => false,
            Some(cell) => match cell {
                Cell::Rock => false,
                Cell::Sand => false,
                Cell::Air => true,
            },
        }
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Polyline {
    points: Vec<Coord>,
}

impl Polyline {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn add(&mut self, coord: Coord) {
        self.points.push(coord);
    }

    fn get_all_coords(&self) -> HashSet<Coord> {
        let mut points: HashSet<Coord> = HashSet::new();
        for couple in self.points.windows(2) {
            let dx = couple[0].x - couple[1].x;
            let dy = couple[0].y - couple[1].y;
            dbg!(dx, dy);
            for i in 0..=dx.abs() {
                if dx > 0 {
                    points.insert((couple[0].x - i, couple[0].y).into());
                } else {
                    points.insert((couple[0].x + i, couple[0].y).into());
                }
            }
            for i in 0..=dy.abs() {
                if dy > 0 {
                    points.insert((couple[0].x, couple[0].y - i).into());
                } else {
                    points.insert((couple[0].x, couple[0].y + i).into());
                }
            }
        }
        points
    }
}

fn parse_line(line: String) -> Polyline {
    let mut polyline = Polyline::new();
    let points: Vec<&str> = line.split("->").collect();
    for point in &points {
        let coord: Vec<&str> = point.trim().split(',').collect();
        polyline.add((coord[0].parse().unwrap(), coord[1].parse().unwrap()).into());
    }
    polyline
}

fn run(input: Vec<String>) -> usize {
    let rocks: Vec<Polyline> = input
        .iter()
        .map(|line| parse_line(line.to_string()))
        .collect();

    dbg!(&rocks);

    let max_x = rocks
        .iter()
        .flat_map(|p| &p.points)
        .map(|c| c.x)
        .max()
        .unwrap();

    let max_y = rocks
        .iter()
        .flat_map(|p| &p.points)
        .map(|c| c.y)
        .max()
        .unwrap();

    dbg!(&max_x, &max_y);

    let mut map = Map::new(max_x as usize + 1, max_y as usize + 1);

    for poly in rocks.iter() {
        for rock in poly.get_all_coords() {
            dbg!(&rock);
            let place = map.get_mut_cell(rock).unwrap();
            *place = Cell::Rock;
        }
    }

    let origin = Coord::new(500, 0);
    let mut sand = origin;
    let mut nb_sand = 0;

    loop {
        if sand.y >= max_y {
            dbg!(nb_sand);
            break;
        }
        match (
            sand.check_left(&map),
            sand.check_down(&map),
            sand.check_right(&map),
        ) {
            (_, true, _) => sand.move_down(),
            (true, false, _) => {
                sand.move_left();
                sand.move_down();
            }
            (false, false, true) => {
                sand.move_right();
                sand.move_down();
            }
            (false, false, false) => {
                let s = map.get_mut_cell(sand).unwrap();
                *s = Cell::Sand;
                sand = origin;
                nb_sand += 1;
            }
        }
    }
    dbg!(&map);

    nb_sand
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
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 24);
    }
}
