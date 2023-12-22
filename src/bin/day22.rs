use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    fs::read_to_string,
};

use itertools::Itertools;

type Pos3 = (usize, usize, usize);
fn overlaps(a: (Pos3, Pos3), b: (Pos3, Pos3)) -> bool {
    a.0.0.max(b.0.0) <= a.1.0.min(b.1.0)
        && a.0.1.max(b.0.1) <= a.1.1.min(b.1.1)
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let mut blocks = lines
        .iter()
        .map(|l| l.split_once("~").unwrap())
        .map(|(f, t)| {
            let f: Pos3 = f
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let t: Pos3 = t
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (f, t)
        })
        .collect::<Vec<_>>();
    blocks.sort_by_key(|(f, _)| f.2);
    println!("Blocks: {:?}", blocks.len());
    let mut blocks_fallen = Vec::new();
    for (i, (f1, t1)) in blocks.iter().enumerate() {
        let mut max_h = 1;
        for (j, (f2, t2)) in blocks_fallen.iter().enumerate() {
            if j > i {
                continue;
            }
            if overlaps((*f1, *t1), (*f2, *t2)) {
                max_h = max_h.max(t2.2 + 1);
            }
        }
        let t1 = (t1.0, t1.1, t1.2 - f1.2 + max_h);
        let f1 = (f1.0, f1.1, max_h);
        blocks_fallen.push((f1, t1));
    }
    blocks_fallen.sort_by_key(|(f, _)| f.2);
    // for b in blocks_fallen.iter().rev() {
    //     println!("{:?}", b);
    // }

    let mut supported_by = HashMap::new();
    let mut supported = HashMap::new();
    blocks_fallen.iter().for_each(|b| {
        supported_by.entry(b).or_insert(HashSet::new());
        supported.entry(b).or_insert(HashSet::new());
    });
    for (i, block) in blocks_fallen.iter().enumerate() {
        for (j, other) in blocks_fallen.iter().enumerate() {
            if j >= i {
                continue;
            }
            if overlaps(*block, *other) && block.0.2 == other.1.2 + 1{
                supported_by.get_mut(other).unwrap().insert(block);
                supported.get_mut(block).unwrap().insert(other);
            }
        }
    }

    // for (k, v) in supported_by.iter() {
    //     println!("{:?}: {:?}", k, v);
    // }
    let mut acc = 0;
    for block in blocks_fallen.iter() {
        if supported_by[block].iter().all(|b| supported[b].len() > 1) {
            acc += 1;
        }
    }
    println!("{}", acc);
    let mut acc2 = 0;
    for block in blocks_fallen.iter() {
        let mut fall = VecDeque::new();
        for &b in supported_by[block].iter().filter(|&b| supported[b].len() == 1) {
            fall.push_back(b);
        }
        let mut falling: HashSet<_> = HashSet::new();
        for b in fall.iter() {
            falling.insert(*b);
        }
        falling.insert(block);
        while let Some(b) = fall.pop_front() {
            let inter = supported_by[&b].difference(&falling).map(|f| *f).collect::<Vec<_>>();
            for k in inter {
                if supported[k].iter().all(|b| falling.contains(b)) {
                    fall.push_back(k);
                    falling.insert(k);
                }
            }
        }
        acc2 += falling.len() - 1;
    }
    println!("{}", acc2);
}
