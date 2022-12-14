use petgraph::algo::dijkstra;
use petgraph::dot::Dot;
use petgraph::prelude::*;
use std::fmt::{self, Debug};

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
        Self {
            height,
            width,
            data: Vec::new(),
        }
    }

    fn push(&mut self, cell: Cell) {
        self.data.push(cell);
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
        let mut line_marker = Vec::new();
        let mut line = Vec::new();
        writeln!(f, "{}x{} grid:", self.width, self.height)?;
        for y in 0..self.height {
            line_marker.clear();
            line.clear();
            for x in 0..self.width {
                let cell = self
                    .get_cell((x.try_into().unwrap(), y.try_into().unwrap()).into())
                    .unwrap();
                let c = match cell {
                    Cell::Start => 'S',
                    Cell::End => 'E',
                    Cell::Elevation(elevation) => (b'a' + *elevation as u8) as char,
                };
                line_marker.push("-");
                line.push(c.to_string());
            }
            writeln!(f, "|{}|", line_marker.join("+"))?;
            write!(f, "|{}|", line.join("|"))?;
            writeln!(f)?;
        }
        writeln!(f, "|{}|", line_marker.join("+"))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Start,
    End,
    Elevation(isize),
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

fn parse_map(input: Vec<String>) -> Map {
    let mut map = Map::new(input[0].chars().count(), input.len());
    for y in input.iter() {
        for value in y.chars() {
            let cell = match value {
                'S' => Cell::Start,
                'E' => Cell::End,
                'a'..='z' => {
                    let elevation = value as u8 - b'a';
                    Cell::Elevation(elevation as isize)
                }
                _ => unreachable!(),
            };
            map.push(cell);
        }
    }
    map
}

fn run(input: Vec<String>) -> usize {
    let mut map = parse_map(input);
    let mut start: Coord = Coord::new(0, 0);
    let mut end: Coord = Coord::new(0, 0);
    let mut edges: Vec<(Coord, Coord)> = Vec::new();
    let neighbors: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for y in 0..map.height {
        for x in 0..map.width {
            let cell = map
                .get_mut_cell((x.try_into().unwrap(), y.try_into().unwrap()).into())
                .unwrap();
            match cell {
                Cell::Start => {
                    start = Coord::new(x.try_into().unwrap(), y.try_into().unwrap());
                    *cell = Cell::Elevation(0)
                }
                Cell::End => {
                    end = Coord::new(x.try_into().unwrap(), y.try_into().unwrap());
                    *cell = Cell::Elevation(25)
                }
                _ => (),
            }
        }
    }
    dbg!(&map);
    dbg!(start, end);

    for neighbor in neighbors {
        for y in 0..map.height {
            for x in 0..map.width {
                let x = isize::try_from(x).unwrap();
                let y = isize::try_from(y).unwrap();
                let nx = x + neighbor.0;
                let ny = y + neighbor.1;

                let cell = map.get_cell((x, y).into()).unwrap();

                let neighbor_cell = map.get_cell((nx, ny).into());

                match neighbor_cell {
                    None => continue,
                    Some(ncell) => match (cell, ncell) {
                        (Cell::Elevation(ec), Cell::Elevation(enc)) => {
                            if enc - ec > 1 {
                                continue;
                            } else {
                                edges.push(((x, y).into(), (nx, ny).into()))
                            };
                        }
                        _ => unreachable!(),
                    },
                }
            }
        }
    }

    let g = DiGraphMap::<Coord, ()>::from_edges(edges.iter().map(|(a, b)| (*b, *a)));
    println!("{:?}", Dot::new(&g));

    let res = dijkstra(&g, end, None, |_| 1);
    dbg!(&res);

    let distances_from_end_to_cell_elev_0 = res
        .iter()
        .filter_map(
            |(coord, dist)| match map.get_cell((coord.x, coord.y).into()) {
                Some(Cell::Elevation(0)) => Some(dist),
                Some(_) => None,
                None => None,
            },
        )
        .collect::<Vec<&usize>>();

    dbg!(&distances_from_end_to_cell_elev_0);

    **distances_from_end_to_cell_elev_0.iter().min().unwrap()
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
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 29);
    }
}
