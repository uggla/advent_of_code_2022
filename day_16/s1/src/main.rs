use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::GraphMap;
use petgraph::prelude::*;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone)]
struct State {
    max_duration: u32,
    elapsed: u32,
    flow_rate: u32,
    released: u32,
    pos: String,
    valves_closed: Vec<Valve>,
    valves_opened: Vec<Valve>,
}

impl State {
    fn new(valves_to_open: Vec<Valve>) -> Self {
        Self {
            max_duration: 30,
            elapsed: 1,
            flow_rate: 0,
            released: 0,
            pos: "AA".to_string(),
            valves_closed: valves_to_open,
            valves_opened: Vec::new(),
        }
    }

    fn next(&mut self) {
        self.elapsed += 1;
        self.released += self.flow_rate;
    }

    fn walk_and_open(&mut self, valve: &str, dist: u32) {
        for _ in 0..dist {
            self.next();
        }
        // One more to open
        self.next();
        // Update state
        self.pos = valve.to_string();
        self.flow_rate += self
            .valves_closed
            .iter()
            .find(|v| v.name == valve)
            .unwrap()
            .flow_rate;
        let valve = self.valves_closed.remove(
            self.valves_closed
                .iter()
                .position(|v| v.name == valve)
                .unwrap(),
        );

        self.valves_opened.push(valve);
    }

    fn run(&mut self, valve_dest: &str, valve_dist: u32) {
        // check if we can do the move
        // no we can not do the move so wait for time out.
        if self.elapsed + valve_dist > self.max_duration {
            while self.elapsed <= self.max_duration {
                self.next();
            }
        } else {
            // yes we can do the move so apply it
            self.walk_and_open(valve_dest, valve_dist);
        }
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

    let mut simulations: VecDeque<State> = VecDeque::new();
    let mut solutions: VecDeque<State> = VecDeque::new();
    // let mut seen = HashSet::new();
    let initial_state = State::new(valves_to_open.clone());

    simulations.push_back(initial_state);

    while !simulations.is_empty() {
        let state = simulations.pop_front().unwrap();
        // dbg!(&state);
        // Which valves are the closest ?
        let closest_valves = distance(&graph, &state.pos, &state.valves_closed);
        // dbg!(&closest_valves);
        for valve in &closest_valves {
            // copy state
            let mut new_state = state.clone();
            // run simulation to get new state
            new_state.run(&valve.0, valve.1);

            // Are  we out of time ?
            if new_state.elapsed > new_state.max_duration {
                // Yes -> push state in solution
                solutions.push_back(new_state);
            } else {
                // No -> push state in simulation to run a new one
                // if seen.insert((
                //     new_state.valves_opened.clone(),
                //     new_state.elapsed,
                //     new_state.released,
                // )) {
                simulations.push_back(new_state);
                // }
            }
        }
        // dbg!(&simulations);
    }

    // dbg!(&simulations);
    // dbg!(&solutions);
    let solution = solutions.iter().map(|s| s.released).max().unwrap();
    dbg!(solution as usize)
    // todo!();
}

fn distance(
    graph: &GraphMap<&str, (), Directed>,
    pos: &str,
    valves_to_open: &Vec<Valve>,
) -> Vec<(String, u32)> {
    let dist = dijkstra(graph, pos, None, |_| 1);
    let mut closest_valves: Vec<_> = valves_to_open
        .iter()
        .filter_map(|v| dist.get_key_value(&v.name.as_ref()))
        .collect();
    if closest_valves.is_empty() {
        // There is no more valve to open return a valve really far
        return vec![("WAIT".to_string(), 1000000)];
    };
    closest_valves.sort_by_key(|s| s.1);
    // Keep only the shortest ones
    // let shortest_val = closest_valves[0].1;
    // closest_valves.retain(|v| v.1 == shortest_val);
    closest_valves
        .iter()
        .map(|v| (v.0.to_string(), *v.1))
        .collect()
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
