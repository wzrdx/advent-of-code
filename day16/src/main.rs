use common::read_lines;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
struct ValveProps {
    flow_rate: i32,
    connections: Vec<String>,
}

#[derive(Debug, Clone)]
struct Valve {
    props: ValveProps,
    dist: HashMap<String, i32>,
}

// Day #16
pub fn main() {
    let lines: Vec<String> = read_lines("day16/input.txt");

    let mut valve_props: HashMap<String, ValveProps> = HashMap::new();
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for line in lines {
        let (name, props) = parse_valve(line.as_str()).expect("Unable to parse line.");
        valve_props.insert(name, props);
    }

    for name in valve_props.keys() {
        let dist = bfs(name, &valve_props);

        valves.insert(
            name.clone(),
            Valve {
                props: valve_props[name].clone(),
                dist,
            },
        );
    }

    find_path(&valves);
}

fn bfs(start: &String, valve_props: &HashMap<String, ValveProps>) -> HashMap<String, i32> {
    let mut dist: HashMap<String, i32> = HashMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut current: String;

    dist.insert(start.clone(), 0);
    queue.push_back(start.clone());

    while !queue.is_empty() {
        current = queue.pop_front().unwrap();

        for neighbor in valve_props[&current].connections.iter() {
            if !dist.contains_key(neighbor) {
                dist.insert(neighbor.clone(), dist[&current] + 1);
                queue.push_back(neighbor.clone());
            }
        }
    }

    dist
}

fn find_path(valves: &HashMap<String, Valve>) {
    let mut open_valves: Vec<String> = Vec::new();
    let useful_valves = valves
        .iter()
        .filter(|(_, v)| v.props.flow_rate > 0)
        .map(|(k, _)| k.clone())
        .collect::<Vec<String>>();

    let mut memo: HashMap<String, i32> = HashMap::new();

    let total_pressure = dfs(
        &"AA".to_string(),
        30,
        &mut open_valves,
        &useful_valves,
        &mut memo,
        &valves,
    );

    println!("Total pressure: {}", total_pressure);
}

fn dfs(
    current_valve: &String,
    minutes_left: i32,
    open_valves: &mut Vec<String>,
    useful_valves: &Vec<String>,
    memo: &mut HashMap<String, i32>,
    valves: &HashMap<String, Valve>,
) -> i32 {
    let mut sorted_open = open_valves.clone();
    sorted_open.sort();
    let key = format!(
        "{}-{}-{}",
        current_valve,
        minutes_left,
        sorted_open.join(",")
    );

    if memo.contains_key(key.as_str()) {
        return memo[key.as_str()];
    }

    let mut best: i32 = 0;

    for valve in useful_valves {
        if open_valves.contains(valve) {
            continue;
        }

        let distance = valves[current_valve].dist[valve] as i32;
        let time_left: i32 = minutes_left - distance - 1;

        if time_left <= 0 {
            continue;
        }

        let flow_rate = valves[valve].props.flow_rate;
        let pressure_released = flow_rate * time_left;

        // Try to see if the result is satisfying by opening the valve
        open_valves.push(valve.clone());

        let total_pressure =
            pressure_released + dfs(valve, time_left, open_valves, useful_valves, memo, valves);

        open_valves.pop(); // Undo that choice before trying the next valve

        best = best.max(total_pressure);
    }

    memo.insert(key.clone(), best);

    best
}

fn parse_valve(line: &str) -> Result<(String, ValveProps), String> {
    let regex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
        .map_err(|e| format!("Invalid regex: {}.", e))?;

    let captures = regex
        .captures(line)
        .ok_or_else(|| "Regex not matching.".to_string())?;

    let name = captures
        .get(1)
        .ok_or("Missing valve name.")?
        .as_str()
        .to_string();

    let flow_rate = captures
        .get(2)
        .ok_or("Missing flow rate.")?
        .as_str()
        .parse::<i32>()
        .map_err(|e| format!("Invalid flow rate: {}.", e))?;

    let connections = captures
        .get(3)
        .ok_or("Missing connections.")?
        .as_str()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok((
        name,
        ValveProps {
            flow_rate,
            connections,
        },
    ))
}
