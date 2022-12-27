#![allow(dead_code)]
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
                data.push(Cell::Nosignal);
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

    fn write_cell(&mut self, coord: Coord, value: Cell) -> Option<()> {
        if !self.bound(&coord) {
            return None;
        }
        if value == Cell::Signal {
            match self.data[coord.y as usize * self.width + coord.x as usize] {
                Cell::Nosignal => {
                    self.data[coord.y as usize * self.width + coord.x as usize] = value;
                    return Some(());
                }
                Cell::Signal => return Some(()),
                Cell::Beacon => return Some(()),
                Cell::Sensor => return Some(()),
                Cell::Origin => {
                    self.data[coord.y as usize * self.width + coord.x as usize] = value;
                    return Some(());
                }
            };
        }
        self.data[coord.y as usize * self.width + coord.x as usize] = value;
        Some(())
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
                    Cell::Signal => '#',
                    Cell::Nosignal => '.',
                    Cell::Sensor => 'S',
                    Cell::Beacon => 'B',
                    Cell::Origin => '*',
                };
                line.push(c.to_string());
            }
            write!(f, "{}", line.join(""))?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Signal,
    Nosignal,
    Sensor,
    Beacon,
    Origin,
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

    fn mdist(&self, other: &Coord) -> usize {
        ((other.x - self.x).abs() + (other.y - self.y).abs())
            .try_into()
            .unwrap()
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Sensor {
    cell: Coord,
    beacon_cell: Coord,
}

impl Sensor {
    fn get_covered_cell(&self) -> Vec<Coord> {
        let mut res: Vec<Coord> = Vec::new();
        let mdist = self.cell.mdist(&self.beacon_cell);
        for y in self.cell.y - mdist as isize..=self.cell.y + mdist as isize {
            for x in self.cell.x - mdist as isize..=self.cell.x + mdist as isize {
                let current_cell = Coord::new(x, y);
                if current_cell.mdist(&self.cell) <= mdist {
                    res.push(current_cell);
                }
            }
        }

        res
    }

    fn get_covered_cell_y(&self, y_to_find: isize) -> Vec<Coord> {
        let mut res: Vec<Coord> = Vec::new();
        let mdist = self.cell.mdist(&self.beacon_cell);

        if (self.cell.y - mdist as isize..=self.cell.y + mdist as isize).contains(&y_to_find) {
            for x in self.cell.x - mdist as isize..=self.cell.x + mdist as isize {
                let current_cell = Coord::new(x, y_to_find);
                if current_cell.mdist(&self.cell) <= mdist {
                    res.push(current_cell);
                }
            }
        }

        res
    }
}

fn parse_line(line: String) -> Sensor {
    let line_csv = line
        .replace("Sensor at x=", "")
        .replace(": closest beacon is at x=", ",")
        .replace("y=", "");

    let coordinates: Vec<&str> = line_csv.split(',').collect();
    let sensor = Sensor {
        cell: (
            coordinates[0].trim().parse().unwrap(),
            coordinates[1].trim().parse().unwrap(),
        )
            .into(),
        beacon_cell: (
            coordinates[2].trim().parse().unwrap(),
            coordinates[3].trim().parse().unwrap(),
        )
            .into(),
    };
    sensor
}

fn get_min_max_coord(sensors: &[Sensor]) -> (isize, isize, isize, isize) {
    let mdist = sensors
        .iter()
        .map(|sensor| sensor.cell.mdist(&sensor.beacon_cell))
        .max()
        .unwrap();
    dbg!(mdist);
    let mut max_x = sensors.iter().map(|sensor| sensor.cell.x).max().unwrap() + mdist as isize;
    let mut min_x = sensors.iter().map(|sensor| sensor.cell.x).min().unwrap() - mdist as isize;
    let mut max_y = sensors.iter().map(|sensor| sensor.cell.y).max().unwrap() + mdist as isize;
    let mut min_y = sensors.iter().map(|sensor| sensor.cell.y).min().unwrap() - mdist as isize;
    let bmax_x = sensors
        .iter()
        .map(|sensor| sensor.beacon_cell.x)
        .max()
        .unwrap();
    let bmin_x = sensors
        .iter()
        .map(|sensor| sensor.beacon_cell.x)
        .min()
        .unwrap();
    let bmax_y = sensors
        .iter()
        .map(|sensor| sensor.beacon_cell.y)
        .max()
        .unwrap();
    let bmin_y = sensors
        .iter()
        .map(|sensor| sensor.beacon_cell.y)
        .min()
        .unwrap();

    if max_x < bmax_x {
        max_x = bmax_x;
    }

    if max_y < bmax_y {
        max_y = bmax_y;
    }

    if min_x > bmin_x {
        min_x = bmin_x;
    }

    if min_y > bmin_y {
        min_y = bmin_y;
    }

    (min_x, max_x, min_y, max_y)
}

#[derive(Debug)]
struct Wrapper {
    min_x: isize,
    min_y: isize,
}

impl Wrapper {
    fn new(min_x: isize, min_y: isize) -> Self {
        Self { min_x, min_y }
    }

    fn wrap(&self, coord: Coord) -> Coord {
        Coord::new(coord.x + self.min_x.abs(), coord.y + self.min_y.abs())
    }
}

fn run(input: Vec<String>, y_to_find: isize) -> usize {
    let sensors: Vec<Sensor> = input
        .iter()
        .map(|line| parse_line(line.to_string()))
        .collect();

    // Uncomment folowing block and comment next one then
    // run cargo test to display map

    // let min_max = dbg!(get_min_max_coord(&sensors));
    //
    // let mut map = Map::new(
    //     (min_max.1 - min_max.0) as usize + 1,
    //     (min_max.3 - min_max.2) as usize + 1,
    // );
    //
    // let wrapper = Wrapper::new(min_max.0, min_max.2);
    // // Place sensor and beacons
    // for sensor in &sensors {
    //     map.write_cell(wrapper.wrap(sensor.cell), Cell::Sensor)
    //         .unwrap();
    //     map.write_cell(wrapper.wrap(sensor.beacon_cell), Cell::Beacon)
    //         .unwrap();
    // }
    //
    // let c = &sensors[6].get_covered_cell();
    // for cell in c {
    //     map.write_cell(wrapper.wrap(*cell), Cell::Signal).unwrap();
    // }
    //
    // map.write_cell(wrapper.wrap((0, 0).into()), Cell::Origin)
    //     .unwrap();
    // dbg!(&map);
    // todo!();

    let truc = sensors
        .iter()
        .flat_map(|sensor| {
            let mut covered = sensor.get_covered_cell_y(y_to_find);
            if sensor.cell.y == y_to_find {
                covered.push(sensor.cell);
            }
            if sensor.beacon_cell.y == y_to_find {
                covered.push(sensor.beacon_cell);
            }
            covered
        })
        .collect::<HashSet<Coord>>();

    dbg!(&truc.len() - 1)
}

fn main() {
    let input = parse_input(None);

    let answer = run(input, 2000000);

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
    fn test_mdist() {
        let cell1 = Coord::new(0, 0);
        let cell2 = Coord::new(10, 0);

        assert_eq!(cell1.mdist(&cell2), 10);

        let cell1 = Coord::new(0, 0);
        let cell2 = Coord::new(10, 1);

        assert_eq!(cell1.mdist(&cell2), 11);

        let cell1 = Coord::new(-5, -5);
        let cell2 = Coord::new(5, 10);

        assert_eq!(cell1.mdist(&cell2), 25);
    }
    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
            "
        )));
        dbg!(&input);
        let answer = run(input, 10);
        assert_eq!(answer, 26);
    }
}
