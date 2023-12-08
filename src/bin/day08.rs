use std::{collections::HashMap, env::args, fs::read_to_string};

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let (instructions, nodes) = string.split_once("\n\n").unwrap();
    let instructions = instructions.chars().collect::<Vec<_>>();
    let nodes = nodes
        .lines()
        .map(|l| l.split_once(" = ").unwrap())
        .map(|(k, v)| (k, v[1..9].split_once(", ").unwrap()))
        .collect::<HashMap<_, _>>();
    // println!("{:?}", instructions);
    // println!("{:?}", nodes);
    let mut current = "AAA";
    let mut ind = 0;
    while current != "ZZZ" {
        let next = instructions[ind % instructions.len()];
        match next {
            'L' => {
                current = nodes[current].0;
            }
            'R' => {
                current = nodes[current].1;
            }
            _ => unreachable!("Invalid instruction"),
        }
        ind += 1;
    }
    println!("{}", ind);

    let current2 = nodes
        .iter()
        .filter(|(k, _)| k.chars().last().unwrap() == 'A')
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();

    let mut memos: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    for start in current2 {
        let mut current = start;
        let mut end_visited = Vec::new();
        let mut ind = 0;
        while end_visited.len() < 2 {
            let next = instructions[ind % instructions.len()];
            match next {
                'L' => current = nodes[current].0,
                'R' => current = nodes[current].1,
                _ => unreachable!("Invalid instruction"),
            }
            if current.chars().into_iter().last().unwrap() == 'Z' {
                memos
                    .entry(start)
                    .or_insert(Vec::new())
                    .push((current, ind));
                end_visited.push(current);
            }
            ind += 1;
        }
    }

    let mut lcm = 1;
    // println!("");
    for (k, v) in memos.iter() {
        let cycle = v[1].1 - v[0].1;
        // println!("{} {:?} {}", k, v, cycle);
        let num = cycle;
        lcm = lcm * num / gcd(lcm, num);
    }
    println!("{}", lcm);
}
