use std::collections::{HashMap, HashSet, VecDeque};
use std::env::args;
use std::fs::read_to_string;

fn print_map(x: (isize, isize), y: (isize, isize), seen: &HashSet<(isize, isize)>) {
    for i in x.0..=x.1 {
        for j in y.0..=y.1 {
            if i == 0 && j == 0 {
                print!("O");
            } else if seen.contains(&((i, j))) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let dirs: HashMap<char, (isize, isize)> =
        HashMap::from([('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))]);

    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let mut current_pos = (0, 0);
    let mut border = HashSet::new();
    for line in lines {
        let mut iter = line.trim().split_whitespace();
        let (dir, amount): (char, usize) = (
            iter.next().unwrap().chars().last().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        let dir = dir;
        let amount = amount;
        let dir_val = dirs[&dir];
        for _ in 0..amount {
            current_pos = (current_pos.0 + dir_val.0, current_pos.1 + dir_val.1);
            border.insert(current_pos);
        }
        // let (dir, amount, color) = tmp.();
    }
    let x_min = *border.iter().map(|(x, _)| x).min().unwrap() as isize;
    let x_max = *border.iter().map(|(x, _)| x).max().unwrap() as isize;
    let y_min = *border.iter().map(|(_, y)| y).min().unwrap() as isize;
    let y_max = *border.iter().map(|(_, y)| y).max().unwrap() as isize;
    print_map((x_min, x_max), (y_min, y_max), &border);
    let mut inside = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, 10));
    while to_visit.len() > 0 {
        let current = to_visit.pop_front().unwrap();
        if inside.contains(&current) || border.contains(&current) {
            continue;
        }
        inside.insert(current);
        for dir in dirs.values() {
            let new_pos = (current.0 + dir.0, current.1 + dir.1);
            to_visit.push_back(new_pos);
        }
    }
    println!("{}", inside.len() + border.len());
}
