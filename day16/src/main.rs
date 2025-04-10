use std::collections::HashMap;

use common::read_lines;
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u32,
    connections: Vec<String>,
}

#[derive(PartialEq)]
enum Action {
    Open,
    Move,
}

// Day #16
pub fn main() {
    let lines: Vec<String> = read_lines("day16/input.txt");
    let mut valves: HashMap<String, Valve> = HashMap::new();

    lines
        .iter()
        .for_each(|line| parse_valve(line.as_str(), &mut valves).expect("Unable to parse line."));

    let mut max_pressure: u32 = 0;
    let mut max_path: Vec<String> = Vec::new();

    for _ in 0..10_000 {
        let (pressure, path) = find_path(&valves);

        if pressure > max_pressure {
            max_pressure = pressure;
            max_path = path;
        }
    }

    println!("{:?}", max_pressure);

    for action in max_path {
        println!("{}", action);
    }
}

fn parse_valve(line: &str, valves: &mut HashMap<String, Valve>) -> Result<(), String> {
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
        .parse::<u32>()
        .map_err(|e| format!("Invalid flow rate: {}.", e))?;

    let connections = captures
        .get(3)
        .ok_or("Missing connections.")?
        .as_str()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    valves.insert(
        name,
        Valve {
            flow_rate,
            connections,
        },
    );

    Ok(())
}

fn find_path(valves: &HashMap<String, Valve>) -> (u32, Vec<String>) {
    let mut open_valves: Vec<String> = Vec::new();
    let mut minutes_left: u32 = 30;
    let mut pressure_released: u32 = 0;
    let mut current_valve: String = "AA".to_string();

    let mut moves: Vec<String> = Vec::new();

    while minutes_left > 0 {
        minutes_left -= 1;

        let valve = valves
            .get(&current_valve)
            .expect("Valve not found in the map.");

        let action = if valve.flow_rate == 0 || open_valves.contains(&current_valve) {
            Action::Move
        } else {
            if rand::thread_rng().gen_bool(0.5) {
                Action::Open
            } else {
                Action::Move
            }
        };

        if action == Action::Open {
            open_valves.push(current_valve.clone());
            pressure_released += valve.flow_rate * minutes_left;

            moves.push(format!(
                "[Minute {}] Open {} which will release {} pressure.",
                (minutes_left).abs_diff(30),
                current_valve,
                valve.flow_rate * minutes_left
            ));
        } else {
            let next_valve = valve
                .connections
                .choose(&mut rand::thread_rng())
                .expect("No connections available.")
                .clone();

            current_valve = next_valve;

            moves.push(format!(
                "[Minute {}] Move to {}.",
                (minutes_left).abs_diff(30),
                current_valve
            ));
        }
    }

    (pressure_released, moves)
}
