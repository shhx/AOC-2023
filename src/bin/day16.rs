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

fn print_map(map: &Vec<Vec<char>>, seen: &HashSet<(usize, usize)>) {
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if seen.contains(&((i, j))) {
                print!("#");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}

fn bounce(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    dir_en: Dir,
    to_visit: &mut VecDeque<((usize, usize), Dir)>,
) {
    match map[pos.0][pos.1] {
        '.' => to_visit.push_back((pos, dir_en)),
        '|' => match dir_en {
            Dir::Up | Dir::Down => to_visit.push_back((pos, dir_en)),
            Dir::Left | Dir::Right => {
                to_visit.push_back((pos, Dir::Up));
                to_visit.push_back((pos, Dir::Down));
            }
        },
        '-' => match dir_en {
            Dir::Left | Dir::Right => to_visit.push_back((pos, dir_en)),
            Dir::Up | Dir::Down => {
                to_visit.push_back((pos, Dir::Left));
                to_visit.push_back((pos, Dir::Right));
            }
        },
        '/' => match dir_en {
            Dir::Up => to_visit.push_back((pos, Dir::Right)),
            Dir::Down => to_visit.push_back((pos, Dir::Left)),
            Dir::Left => to_visit.push_back((pos, Dir::Down)),
            Dir::Right => to_visit.push_back((pos, Dir::Up)),
        },
        '\\' => match dir_en {
            Dir::Up => to_visit.push_back((pos, Dir::Left)),
            Dir::Down => to_visit.push_back((pos, Dir::Right)),
            Dir::Left => to_visit.push_back((pos, Dir::Up)),
            Dir::Right => to_visit.push_back((pos, Dir::Down)),
        },
        _ => unreachable!(),
    }
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

    let mut seen = HashSet::new();
    let mut to_visit: VecDeque<((usize, usize), Dir)> = VecDeque::new();
    bounce(&map, (0, 0), Dir::Right, &mut to_visit);
    while to_visit.len() > 0 {
        let (pos, dir_en) = to_visit.pop_front().unwrap();
        // println!("{:?}", (pos, dir_en));
        if seen.contains(&(pos, dir_en)) {
            continue;
        }
        seen.insert((pos, dir_en));
        let dir = dirs[&dir_en];
        let next = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        if next.0 < 0
            || next.1 < 0
            || next.0 >= map.len() as isize
            || next.1 >= map[0].len() as isize
        {
            continue;
        }
        let pos = (next.0 as usize, next.1 as usize);
        bounce(&map, pos, dir_en, &mut to_visit);
    }
    let mut positions = HashSet::new();
    for (pos, _) in seen {
        positions.insert(pos);
    }
    print_map(&map, &positions);
    println!("{:?}", positions.len());
}
