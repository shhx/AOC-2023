use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    fs::read_to_string,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

type Pos = (usize, usize);

fn print_map(rocks: &Vec<Pos>, ends: &HashSet<Pos>, x_len: usize, y_len: usize) {
    println!("------------");
    let mut map = vec![vec!['.'; y_len]; x_len];
    for (x, y) in rocks {
        map[*x][*y] = '#';
    }
    for (x, y) in ends {
        map[*x][*y] = 'O';
    }
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
    println!("------------");
}

fn main() {
    let dirs: HashMap<Dir, (isize, isize)> = HashMap::from([
        (Dir::Up, (-1, 0)),
        (Dir::Down, (1, 0)),
        (Dir::Left, (0, -1)),
        (Dir::Right, (0, 1)),
    ]);
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let map = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut rocks = Vec::new();
    let mut start = (0, 0);
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 'S' {
                start = (x, y);
            } else if map[x][y] == '#' {
                rocks.push((x, y));
            }
        }
    }
    println!("Start: {:?}", start);
    // println!("Rocks: {:?}", rocks);

    let mut visited = HashSet::new();
    let mut to_visit: VecDeque<_> = VecDeque::new();
    to_visit.push_back((start, 0));
    let max_steps = 64;
    let mut ends_pos = HashSet::new();
    while let Some((pos, steps)) = to_visit.pop_front() {
        if visited.contains(&(pos, steps)) {
            continue;
        }
        if rocks.contains(&pos) {
            continue;
        }
        if steps >= max_steps {
            ends_pos.insert(pos);
            continue;
        }
        visited.insert((pos, steps));
        if pos.0 == 0 || pos.0 == map.len() - 1 || pos.1 == 0 || pos.1 == map[0].len() - 1 {
            println!("Hit border!!!!!!!!!");
            continue;
        }
        for (_, dir) in dirs.iter() {
            let next = ((pos.0 as isize + dir.0) as usize, (pos.1 as isize + dir.1) as usize);
            to_visit.push_back((next, steps + 1));
        }
    }
    print_map(&rocks, &ends_pos, map.len(), map[0].len());
    println!("Ends: {}", ends_pos.len());
}
