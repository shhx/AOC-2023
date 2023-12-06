use std::{
    collections::{HashSet, VecDeque},
    env::args,
    fs::read_to_string,
    iter::zip,
};

fn get_distance(time: usize, pressed: usize) -> usize {
    if pressed == time {
        return 0;
    }
    return (time - pressed) * (time - (time - pressed));
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("{:?} {:?}", times, distances);

    let time2 = times
        .iter()
        .fold("".to_string(), |acc, f| acc + f.to_string().as_str())
        .parse::<usize>()
        .unwrap();
    let distance2 = distances
        .iter()
        .fold("".to_string(), |acc, f| acc + &f.to_string())
        .parse::<usize>()
        .unwrap();
    println!("{} {}", time2, distance2);

    let mut acc = 1;
    for (time, distance) in zip(times, distances) {
        let mut total_ways = 0;
        for pressed in 0..time {
            if get_distance(time, pressed) > distance {
                total_ways += 1;
            }
        }
        acc *= total_ways;
        // println!("ways: {}", total_ways);
    }

    let acc2 = (0..time2)
        .map(|p| get_distance(time2, p))
        .filter(|d| *d > distance2)
        .count();
    println!("{}", acc);
    println!("{}", acc2);
}
