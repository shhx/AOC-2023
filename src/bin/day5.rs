use std::{
    collections::{HashSet, VecDeque},
    env::args,
    fmt::{self, Error, Formatter},
    fs::read_to_string,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Clone)]
struct Map {
    dst: Range,
    src: Range,
}

impl Range {
    fn new(start: usize, range: usize) -> Self {
        Self {
            start,
            end: start + range - 1,
        }
    }
    fn intersection(&self, map: &Map) -> (Range, Option<Vec<Range>>) {
        if self == &map.src {
            return (map.dst.clone(), None);
        }
        if self.start <= map.src.start && self.end >= map.src.end {
            if self.start == map.src.start {
                return (
                    map.dst.clone(),
                    Some(vec![Range {
                        start: map.src.end + 1,
                        end: self.end,
                    }]),
                );
            }
            if self.end == map.src.end {
                return (
                    map.dst.clone(),
                    Some(vec![Range {
                        start: self.start,
                        end: map.src.start - 1,
                    }]),
                );
            }
            return (
                map.dst.clone(),
                Some(vec![
                    Range {
                        start: self.start,
                        end: map.src.start - 1,
                    },
                    Range {
                        start: map.src.end + 1,
                        end: self.end,
                    },
                ]),
            );
        }
        if self.start >= map.src.start && self.end <= map.src.end {
            let cut_dst_start = map.dst.start + self.start - map.src.start;
            let cut_dst_end = map.dst.start + self.end - map.src.start;
            return (
                Range {
                    start: cut_dst_start,
                    end: cut_dst_end,
                },
                None,
            );
        }
        // 2
        if self.end >= map.src.start && self.end <= map.src.end {
            let cut_dst = map.dst.start + self.end - map.src.start;
            return (
                Range {
                    start: map.dst.start,
                    end: cut_dst,
                },
                Some(vec![Range {
                    start: self.start,
                    end: map.src.start - 1,
                }]),
            );
        }
        // 3
        if self.start >= map.src.start && self.start <= map.src.end {
            let cut_dst = map.dst.start + self.start - map.src.start;
            return (
                Range {
                    start: cut_dst,
                    end: map.dst.end,
                },
                Some(vec![Range {
                    start: map.src.end + 1,
                    end: self.end,
                }]),
            );
        }
        return (Range { start: 0, end: 0 }, None);
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[{} -> {}]", self.start, self.end)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "|{} -> {}|", self.src, self.dst)
    }
}

fn is_inside(dst: usize, src: usize, range: usize) -> bool {
    if dst >= src && dst <= src + range {
        return true;
    }
    false
}

fn get_dst(next_dst: usize, dst: usize, src: usize, range: usize) -> usize {
    return next_dst + dst - src;
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let blocks = string
        .split("\n")
        .filter(|x| *x != "")
        .collect::<Vec<&str>>();
    let seeds = blocks[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let seed_ranges = seeds
        .chunks(2)
        .map(|x| Range::new(x[0], x[1]))
        .collect::<Vec<_>>();

    let mut map_sets = Vec::new();
    for block in blocks[1..].iter() {
        if block.contains(":") {
            map_sets.push(Vec::new());
        } else {
            let values = block
                .trim()
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (dst, src, range) = (values[0], values[1], values[2]);
            map_sets.last_mut().unwrap().push((dst, src, range));
        }
    }
    let mut min_dist = usize::MAX;
    for seed in seeds {
        let mut next_dst = seed;
        for map in map_sets.iter() {
            for (dst, src, range) in map.iter() {
                if is_inside(next_dst, *src, *range) {
                    // println!("{} is inside {} {}", next_dst, src, range);
                    next_dst = get_dst(next_dst, *dst, *src, *range);
                    break;
                }
            }
        }
        min_dist = min_dist.min(next_dst);
    }

    // Part 2
    let mut all_ranges = Vec::new();
    for srange in seed_ranges {
        let mut next_ranges = VecDeque::new();
        next_ranges.push_back(srange);
        for (i, map) in map_sets.iter().enumerate() {
            // println!("Next ranges {:?}", next_ranges);
            let mut new_ranges = HashSet::new();
            while next_ranges.len() != 0 {
                let range = next_ranges.pop_front().unwrap();
                let mut did_intersect = false;
                for (dst, src, r) in map.iter() {
                    let m = Map {
                        dst: Range::new(*dst, *r),
                        src: Range::new(*src, *r),
                    };
                    let (intersect, unmapped) = range.intersection(&m);
                    // println!("{} {} -> {} {:?}", range, m, intersect, unmapped);
                    if intersect.start != 0 && intersect.end != 0 {
                        new_ranges.insert(intersect);
                        for range in unmapped.unwrap_or(Vec::new()).iter() {
                            next_ranges.push_back(range.clone());
                        }
                        did_intersect = true;
                        break;
                    }
                }
                if !did_intersect {
                    new_ranges.insert(range);
                }
            }
            next_ranges.clear();
            for range in new_ranges {
                next_ranges.push_back(range);
            }
        }
        all_ranges.extend(next_ranges);
    }
    println!("{}", min_dist);
    println!("{:?}", all_ranges.iter().map(|x| x.start).min().unwrap());
}
