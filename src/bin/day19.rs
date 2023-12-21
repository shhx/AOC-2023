use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

fn main() {
    let regex = Regex::new(r"(\w)([<|>])(\d*)").unwrap();
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let (flows, parts) = string.split_once("\n\n").unwrap();
    let parts: Vec<Vec<_>> = parts
        .lines()
        .map(|l| {
            l.chars()
                .skip(1)
                .take(l.len() - 2)
                .join("")
                .split(",")
                .map(|x| x.split_once("=").unwrap())
                .map(|(x, y)| (x.to_string(), y.parse::<usize>().unwrap()))
                .collect()
        })
        .collect();
    let flows: HashMap<_, _> = flows
        .lines()
        .map(|l| l.split_once('{').unwrap())
        .map(|(id, c)| {
            (
                id,
                c.chars()
                    .take(c.len() - 1)
                    .join("")
                    .to_string()
                    .split(',')
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    println!("{:?}", parts);
    println!("{:?}", flows);
    let mut accepted = HashSet::new();
    for part in parts.iter() {
        let mut flow = &flows["in"];
        let mut end = false;
        while !end {
            // println!("Next flow{:?}", flow);
            for f in flow.iter() {
                // println!("    Flow cond: {:?}", f);
                if let Some((cond, next)) = f.split_once(":") {
                    let caps = regex.captures(cond).unwrap();
                    let p = caps[1].to_string();
                    let op = caps[2].to_string();
                    let n = caps[3].parse::<usize>().unwrap();
                    // println!("    {} {} {} -> {}", p, op, n, next);
                    let out = match op.as_str() {
                        "<" => {
                            if part.iter().any(|(x, y)| x == &p && y < &n) {
                                match next {
                                    "A" => {
                                        accepted.insert(part.clone());
                                        end = true;
                                        true
                                    }
                                    "R" => {
                                        end = true;
                                        true
                                    }
                                    _ => {
                                        flow = &flows[&next];
                                        true
                                    }
                                }
                                // flow = &flows[&next];
                                // true
                            } else {
                                false
                            }
                        }
                        ">" => {
                            if part.iter().any(|(x, y)| x == &p && y > &n) {
                                match next {
                                    "A" => {
                                        accepted.insert(part.clone());
                                        end = true;
                                        true
                                    }
                                    "R" => {
                                        end = true;
                                        true
                                    }
                                    _ => {
                                        flow = &flows[&next];
                                        true
                                    }
                                }
                                // flow = &flows[&next];
                                // true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };
                    if out {
                        break;
                    }
                } else {
                    let out = match f.as_str() {
                        "A" => {
                            accepted.insert(part.clone());
                            end = true;
                            true
                        }
                        "R" => {
                            end = true;
                            true
                        }
                        _ => {
                            flow = &flows[&f.as_str()];
                            true
                        }
                    };
                    if out {
                        break;
                    }
                }
            }
        }
    }
    println!("{:?} {}", accepted, accepted.len());
    let sum = accepted
        .iter()
        .map(|x| x.iter().map(|(_, y)| y).sum::<usize>())
        .sum::<usize>();
    println!("{}", sum);
}
