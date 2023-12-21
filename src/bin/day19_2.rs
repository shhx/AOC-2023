use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

fn calculate(
    part_ranges: &HashMap<String, (usize, usize)>,
    flows: &HashMap<&str, Vec<String>>,
    flow: &str,
) -> usize {
    let reg = Regex::new(r"(\w)([<|>])(\d*)").unwrap();
    let mut sum = 0;
    match flow {
        "A" => {
            let total = part_ranges
                .iter()
                .map(|(_, (min, max))| max - min + 1)
                .fold(1, |acc, x| acc * x);
            // println!("Total: {}", total);
            return total;
        }
        "R" => {
            // println!("Total: {}", 0);
            return 0;
        }
        _ => (),
    }
    let mut part_ranges = part_ranges.clone();
    for f in flows[flow].iter() {
        if let Some((cond, next)) = f.split_once(":") {
            let caps = reg.captures(cond).unwrap();
            let p = caps[1].to_string();
            let op = caps[2].to_string();
            let num = caps[3].parse::<usize>().unwrap();
            // println!("    {} => {:?} {:?}", flow, part_ranges, cond);
            let total = match op.as_str() {
                "<" => {
                    if part_ranges
                        .iter()
                        .any(|(x, (min, max))| x == &p && min < &num && &num <= max)
                    {
                        let new_range1 = (part_ranges[&p].0, num - 1);
                        let rem = (num, part_ranges[&p].1);
                        let mut part_ranges2 = part_ranges.clone();
                        part_ranges2.insert(p.clone(), new_range1);
                        part_ranges.insert(p, rem);
                        // println!("    < {:?} -> {}", part_ranges, next);
                        calculate(&part_ranges2, flows, next)
                    } else {
                        0
                    }
                }
                ">" => {
                    if part_ranges
                        .iter()
                        .any(|(x, (min, max))| x == &p && min <= &num && &num < max)
                    {
                        let new_range1 = (num + 1, part_ranges[&p].1);
                        let rem = (part_ranges[&p].0, num);
                        let mut part_ranges2 = part_ranges.clone();
                        part_ranges2.insert(p.clone(), new_range1);
                        part_ranges.insert(p, rem);
                        // println!("    > {:?} -> {}", part_ranges2, next);
                        calculate(&part_ranges2, flows, next)
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            };
            // println!("Total: {}", total);
            sum += total;
        } else {
            let total = calculate(&part_ranges, flows, f);
            sum += total;
        }
    }
    return sum;
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let (flows, _) = string.split_once("\n\n").unwrap();
    let range = (1, 4000);
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
    let mut part_ranges = HashMap::new();
    part_ranges.insert("x".to_string(), range);
    part_ranges.insert("m".to_string(), range);
    part_ranges.insert("a".to_string(), range);
    part_ranges.insert("s".to_string(), range);
    let total = calculate(&part_ranges, &flows, "in");
    println!("{}", total);
}
