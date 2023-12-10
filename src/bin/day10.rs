use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    fs::read_to_string,
};

fn get_next(pipes: &Vec<Vec<char>>, current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut nexts = Vec::new();
    let (nx, ny) = current;
    match pipes[nx][ny] {
        'J' => {
            if nx > 0 {
                nexts.push((nx - 1, ny));
            }
            if ny > 0 {
                nexts.push((nx, ny - 1));
            }
        }
        'L' => {
            if nx > 0 {
                nexts.push((nx - 1, ny));
            }
            if ny < pipes[0].len() - 1 {
                nexts.push((nx, ny + 1));
            }
        }
        '7' => {
            if nx < pipes.len() - 1 {
                nexts.push((nx + 1, ny));
            }
            if ny > 0 {
                nexts.push((nx, ny - 1));
            }
        }
        'F' => {
            if nx < pipes.len() - 1 {
                nexts.push((nx + 1, ny));
            }
            if ny < pipes[0].len() - 1 {
                nexts.push((nx, ny + 1));
            }
        }
        '|' => {
            if nx > 0 {
                nexts.push((nx - 1, ny));
            }
            if nx < pipes[0].len() - 1 {
                nexts.push((nx + 1, ny));
            }
        }
        '-' => {
            if ny > 0 {
                nexts.push((nx, ny - 1));
            }
            if ny < pipes[0].len() - 1 {
                nexts.push((nx, ny + 1));
            }
        }
        '.' => {}
        _ => unreachable!("Invalid instruction"),
    }
    nexts
}

fn insert_dist(
    distances: &mut HashMap<(usize, usize), usize>,
    next: (usize, usize),
    current_dist: usize,
) {
    distances
        .entry(next)
        .and_modify(|d| {
            if *d > current_dist {
                *d = current_dist;
            }
        })
        .or_insert(current_dist);
}
fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let mut pipes = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut starts = Vec::new();
    let mut start = (0, 0);
    for x in 0..pipes.len() {
        for y in 0..pipes[0].len() {
            if pipes[x][y] == 'S' {
                start = (x, y);
                break;
            }
        }
    }
    let (sx, sy) = start;
    let mut posible_starts = HashSet::from(['J', 'L', '7', 'F', '|', '-']);
    if pipes[sx + 1][sy] == '|' || pipes[sx + 1][sy] == 'J' || pipes[sx + 1][sy] == 'L' {
        starts.push((sx + 1, sy));
        posible_starts = posible_starts.intersection(&HashSet::from(['|', 'F', '7'])).map(|x| *x).collect();
    }
    if sx > 0 && (pipes[sx - 1][sy] == '|' || pipes[sx - 1][sy] == '7' || pipes[sx - 1][sy] == 'F') {
        starts.push((sx - 1, sy));
        posible_starts = posible_starts.intersection(&HashSet::from(['|', 'J', 'L'])).map(|x| *x).collect();
    }
    if pipes[sx][sy + 1] == '-' || pipes[sx][sy + 1] == 'J' || pipes[sx][sy + 1] == '7' {
        starts.push((sx, sy + 1));
        posible_starts = posible_starts.intersection(&HashSet::from(['-', 'F', 'L'])).map(|x| *x).collect();
    }
    if sy > 0 && (pipes[sx][sy - 1] == '-' || pipes[sx][sy - 1] == 'L' || pipes[sx][sy - 1] == 'F') {
        starts.push((sx, sy - 1));
        posible_starts = posible_starts.intersection(&HashSet::from(['-', 'J', '7'])).map(|x| *x).collect();
    }
    assert!(posible_starts.len() == 1);
    println!("Start is {:?}", posible_starts);
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    let mut distances = HashMap::new();
    for start in &starts {
        to_visit.push_back(*start);
        distances.insert(*start, 1);
    }
    visited.insert(start);
    let mut max_dist = 0;
    while to_visit.len() > 0 {
        let next = to_visit.pop_back().unwrap();
        // println!("{:?} -> {:?}", pipes[next.0][next.1], next);
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);
        let (nx, ny) = next;
        let current_dist = distances[&(nx, ny)] + 1;
        let nexts = get_next(&pipes, next);
        if nexts.len() == 0 {
            distances.remove(&next);
            continue;
        }
        for next in nexts {
            to_visit.push_back(next);
            insert_dist(&mut distances, next, current_dist);
        }
        max_dist = max_dist.max(distances[&(nx, ny)]);
    }

    println!("{}", max_dist / 2 + 1);

    pipes[start.0][start.1] = *posible_starts.iter().next().unwrap();

    // Part 2
    let mut tiles_inside = Vec::new();
    for x in 0..pipes.len() {
        let mut inside = false;
        for y in 0..pipes[0].len() {
            // println!("({}, {}) -> {}: {}", x, y, pipes[x][y], inside && !distances.contains_key(&(x, y)));
            if inside && !distances.contains_key(&(x, y)) {
                tiles_inside.push((x, y));
            }
            if distances.contains_key(&(x, y)) && ['|', 'F', '7'].contains(&pipes[x][y]) {
                inside = !inside;
            }
        }
    }
    println!("{}", tiles_inside.len());

    for x in 0..pipes.len() {
        for y in 0..pipes[0].len() {
            if distances.contains_key(&(x, y)) {
                print!("*");
                // print!("{}", pipes[x][y]);
            } else if tiles_inside.contains(&(x, y)) {
                print!("I");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
