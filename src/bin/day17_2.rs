use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    dir: Dir,
    repeat: usize,
    loss: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
    to_visit.push(State {
        pos: (0, 0),
        dir: Dir::Right,
        repeat: 0,
        loss: 0,
    });
    to_visit.push(State {
        pos: (0, 0),
        dir: Dir::Down,
        repeat: 0,
        loss: 0,
    });

    while let Some(State {
        pos,
        dir: dir_en,
        repeat,
        loss,
    }) = to_visit.pop()
    {
        if seen.contains(&(pos, dir_en, repeat)) {
            continue;
        }
        if repeat > 10 {
            continue;
        }
        seen.insert((pos, dir_en, repeat));
        if pos.0 == map.len() - 1 && pos.1 == map[0].len() - 1 && repeat >= 4 {
            println!("{}", loss);
            break;
        }
        if repeat < 4 {
            let repeat = repeat + 1;
            let dir_val = dirs[&dir_en];
            let ne = (pos.0 as isize + dir_val.0, pos.1 as isize + dir_val.1);
            if ne.0 < 0 || ne.1 < 0 || ne.0 >= map.len() as isize || ne.1 >= map[0].len() as isize {
                continue;
            }
            let pos = (ne.0 as usize, ne.1 as usize);
            to_visit.push(State {
                pos,
                dir: dir_en,
                repeat,
                loss: loss + map[pos.0][pos.1],
            });
            continue;
        }

        for dir in dirs.keys() {
            if *dir == dir_en.opposite() {
                continue;
            }
            let dir_val = dirs[&dir];
            let ne = (pos.0 as isize + dir_val.0, pos.1 as isize + dir_val.1);
            if ne.0 < 0 || ne.1 < 0 || ne.0 >= map.len() as isize || ne.1 >= map[0].len() as isize {
                continue;
            }
            let mut repeat = repeat;
            if *dir == dir_en {
                repeat += 1;
            } else {
                repeat = 1;
            }
            let pos = (ne.0 as usize, ne.1 as usize);
            to_visit.push(State {
                pos,
                dir: *dir,
                repeat,
                loss: loss + map[pos.0][pos.1],
            });
        }
    }
}
