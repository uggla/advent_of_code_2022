use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::GraphMap;
use petgraph::{prelude::*, EdgeType};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;

//#![allow(dead_code)]
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

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: String, flow_rate: u32) -> Self {
        Self {
            name,
            flow_rate,
            tunnels: Vec::new(),
        }
    }

    fn add_tunnel(&mut self, name: &str) {
        self.tunnels.push(name.to_string());
    }
}

#[derive(Clone)]
struct State<'a> {
    graph: GraphMap<&'a str, (), Directed>,
    max_duration: u32,
    elapsed: u32,
    flow_rate: u32,
    released: u32,
    pos: String,
    valves_to_open: Vec<Valve>,
}

impl<'a> State<'a> {
    fn new(graph: GraphMap<&'a str, (), Directed>, valves_to_open: Vec<Valve>) -> Self {
        Self {
            graph,
            max_duration: 30,
            elapsed: 1,
            flow_rate: 0,
            released: 0,
            pos: "AA".to_string(),
            valves_to_open,
        }
    }

    fn next(&mut self) {
        self.elapsed += 1;
        self.released += self.flow_rate;
    }

    fn walk_and_open(&mut self, valve: &str) {
        for _ in 0..self.distance(valve) {
            self.next();
        }
        // One more to open
        self.next();
        // Update state
        self.pos = valve.to_string();
        self.flow_rate += self
            .valves_to_open
            .iter()
            .find(|v| v.name == valve)
            .unwrap()
            .flow_rate;
        self.valves_to_open.remove(
            self.valves_to_open
                .iter()
                .position(|v| v.name == valve)
                .unwrap(),
        );
    }

    fn simul(&mut self, valve: &str) -> Option<(u32, String)> {
        self.walk_and_open(valve);
        dbg!(
            &self.elapsed,
            &self.flow_rate,
            &self.released,
            &self.pos,
            &self.valves_to_open
        );
        if self.elapsed > self.max_duration {
            None
        } else {
            while self.elapsed <= self.max_duration {
                self.next();
                dbg!(&self.elapsed, &self.flow_rate, &self.released, &self.pos,);
            }
            Some((self.released, valve.to_string()))
        }
    }

    fn run(&mut self, valve: &str) -> u32 {
        self.walk_and_open(valve);

        todo!();
    }

    fn distance(&self, valve: &str) -> u32 {
        let res = dijkstra(&self.graph, &self.pos, Some(valve), |_| 1);
        res[valve]
    }
}

fn parse_line(line: &str) -> Valve {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let tmp = line
        .replace("Valve ", "")
        .replace(" has flow rate=", ",")
        .replace(" tunnels lead to valves ", "")
        .replace(" tunnel leads to valve ", "");

    let tmp = tmp.split(';').collect::<Vec<&str>>();
    let lhs = tmp[0].split(',').collect::<Vec<&str>>();
    let rhs = tmp[1].split(',').collect::<Vec<&str>>();

    let mut valve = Valve::new(lhs[0].trim().to_string(), lhs[1].trim().parse().unwrap());

    for item in &rhs {
        valve.add_tunnel(item.trim());
    }

    valve
}

fn run(input: Vec<String>) -> usize {
    let valves: Vec<Valve> = input.iter().map(|o| parse_line(o)).collect();
    dbg!(&valves);

    // Build graph
    let mut graph: GraphMap<&str, (), Directed> = GraphMap::new();
    graph.add_edge("BB", "AA", ());

    for valve in &valves {
        for tunnel in &valve.tunnels {
            graph.add_edge(&valve.name, &tunnel, ());
        }
    }

    // Get a dot file
    let mut f = File::create("../graph.dot").unwrap();
    f.write_all(format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).as_bytes())
        .unwrap();

    // It does not make sense to open valves with a 0 rate.
    // Compute all combinations of valves to open could be ok with test data as
    // there are only 6 valves, so 6! = 720 combinations.
    // However, we have 15 valves within self data, so 15! = 1307674368000
    // combinations ! So impossible in a reasonable time.
    // We need to simulate the best choice at each step.

    let valves_to_open = valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.clone())
        .collect::<Vec<Valve>>();

    dbg!(&valves_to_open);
    let mut state = State::new(graph, valves_to_open.clone());
    let sim = valves_to_open
        .clone()
        .iter()
        .map(|v| {
            let mut state_clone = state.clone();
            state_clone.simul(&v.name)
        })
        .collect::<Vec<Option<(u32, String)>>>();

    dbg!(sim);

    // let solutions = valve_names
    //     .iter()
    //     .permutations(valve_names.len())
    //     .collect::<Vec<Vec<&String>>>();
    //
    // dbg!(&solutions.len());

    // let res = dijkstra(&graph, "AA", None, |_| 1);
    todo!();
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
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 1651);
    }
}
