use std::{collections::HashMap, env::args, fs::read_to_string, ops::Index};

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, ch| (acc + ch as usize) * 17 % 256)
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let groups: Vec<_> = string.trim().split(',').collect();
    // println!("{:?}", groups);
    let acc = groups
        .iter()
        .map(|chs| hash(chs))
        .sum::<usize>();
    println!("{}", acc);

    let mut boxes = HashMap::new();
    for group in groups {
        // let chars = group.chars().collect::<Vec<_>>();
        let op_ind: usize;
        if group.chars().last().unwrap().is_numeric() {
            op_ind = group.len() - 2;
        } else {
            op_ind = group.len() - 1;
        }

        let key = group.chars().take(op_ind).collect::<String>();
        let hash = hash(&key);
        // println!("{}", hash);
        match group.chars().nth(op_ind).unwrap() {
            '=' => {
                let b = boxes.entry(hash).or_insert(Vec::new());
                let index = b.iter().position(|(k, _)| k == &key);
                if let Some(i) = index {
                    b[i] = (key, group.chars().nth(op_ind+1).unwrap());
                } else {
                    b.push((key, group.chars().nth(op_ind+1).unwrap()));
                }
            }
            '-' => {
                if boxes.contains_key(&hash) {
                    let b = boxes.get_mut(&hash).unwrap();
                    let index = b.iter().position(|(k, _)| k == &key);
                    if let Some(i) = index {
                        b.remove(i);
                    }
                }
            }
            _ => panic!("Unknown operator"),
        }
        // println!("{:?}", boxes);
    }
    let mut acc2 = 0;
    for (k, v) in boxes {
        if v.len() > 0 {
            for (i, (_, focal)) in v.iter().enumerate() {
                acc2 += (k+1) * (i+1) * focal.to_digit(10).unwrap() as usize;
            }
        }
    }
    println!("{}", acc2);
}
