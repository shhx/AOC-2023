use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn invert(&self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    FF(Pulse),
    Conj(HashMap<String, Pulse>),
    Broad,
}

fn pulse(
    circuit: &mut HashMap<String, (Type, Vec<&str>)>,
    el_name: String,
    from: String,
    input: Pulse,
) -> Option<Pulse> {
    if !circuit.contains_key(&el_name) {
        // println!("{} {input:?} not found!!!!!!!!!!!!!!!!!!!!!!", el_name);
        return None;
    }
    let part = circuit.get_mut(&el_name).unwrap();
    match (&mut part.0, input) {
        (Type::FF(_), Pulse::High) => None,
        (Type::FF(ref mut p), Pulse::Low) => {
            let out = p.clone();
            *p = p.invert();
            Some(out)
        }
        (Type::Conj(ref mut p), _) => {
            p.entry(from).and_modify(|x| *x = input);
            let all = p.iter().all(|(_, v)| *v == Pulse::High);
            if all {
                Some(Pulse::Low)
            } else {
                Some(Pulse::High)
            }
        }
        (Type::Broad, _) => Some(input),
    }
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let mut circuit = HashMap::new();
    for line in string.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        match from.chars().next().unwrap() {
            '%' => {
                let to = to.split(",").map(|x| x.trim()).collect::<Vec<_>>();
                let from = from.split_at(1).1;
                circuit.insert(from.to_string(), (Type::FF(Pulse::High), to));
            }
            '&' => {
                let to = to.split(",").map(|x| x.trim()).collect::<Vec<_>>();
                let from = from.split_at(1).1;
                circuit.insert(from.to_string(), (Type::Conj(HashMap::new()), to));
            }
            _ => {
                let to = to.split(",").map(|x| x.trim()).collect::<Vec<_>>();
                circuit.insert(from.to_string(), (Type::Broad, to));
            }
        }
    }
    let to_insert = circuit
        .iter()
        .map(|(name, (_, to))| {
            let to = to
                .iter()
                .filter(|&x| {
                    if let Some((Type::Conj(_), _)) = circuit.get(*x) {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>();
            (
                to.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                name.clone(),
            )
        })
        .filter(|(x, _)| x.len() > 0)
        .collect::<Vec<_>>();
    for (to, name) in to_insert {
        for t in to.iter() {
            let t = t.to_string();
            let (conj, _) = circuit.get_mut(&t).unwrap();
            if let Type::Conj(c) = conj {
                c.insert(name.to_string(), Pulse::Low);
            }
        }
    }
    println!("{:?}", circuit);
    let original_state = circuit.clone();
    let mut states = Vec::new();
    let to_rx = circuit
        .iter()
        .filter(|(_, (_, v))| v.contains(&"bn"))
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    println!("To rx: {:?}", to_rx);
    let mut cycles: Vec<usize> = Vec::new();
    for target in to_rx {
        let mut cnt = 0;
        let mut circuit = original_state.clone();
        let mut found = false;
        while !found {
            let mut operations = VecDeque::new();
            let mut total_pulses_low = 1;
            let mut total_pulses_high = 0;
            operations.push_back((Pulse::Low, "broadcaster".to_string(), "button".to_string()));
            while operations.len() > 0 {
                let (input, el_name, from) = operations.pop_front().unwrap();
                if el_name == *target && input == Pulse::Low {
                    cycles.push(cnt+1);
                    found = true;
                    break;
                }
                // println!("At: {el_name}:{:?} input: {:?}", circuit[&el_name].0, input);
                if let Some(p) = pulse(&mut circuit, el_name.clone(), from.to_string(), input) {
                    for next in circuit[&el_name].1.iter() {
                        // println!("    {el_name}: {p:?} -> {}", next);
                        operations.push_back((p, next.to_string(), el_name.to_string()));
                        match p {
                            Pulse::Low => total_pulses_low += 1,
                            Pulse::High => total_pulses_high += 1,
                        }
                    }
                }
            }
            // Check state
            if original_state.iter().all(|(k, v)| circuit[k] == *v) {
                println!("Stable state");
                states.push((total_pulses_low, total_pulses_high));
                break;
            }
            // println!("Total pulses: {} {}", total_pulses_low, total_pulses_high);
            states.push((total_pulses_low, total_pulses_high));
            cnt += 1;
        }
    }
    println!("Cycles: {:?}", cycles);
    let acc = cycles.iter().fold(1, |acc, x| acc * x);
    println!("{}", acc);
}
