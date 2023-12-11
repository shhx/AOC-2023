use std::{
    env::args,
    fs::read_to_string,
};

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let space = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut galaxies = Vec::new();
    let mut real_x = 0;
    // let expansion = 1;
    let expansion = 1000000 - 1;
    for x in 0..space.len() {
        let mut empty = true;
        for y in 0..space[0].len() {
            if space[x][y] == '#' {
                galaxies.push((real_x, y));
                empty = false;
            }
        }
        real_x += 1;
        if empty {
            real_x += expansion;
        }
    }
    let mut real_y = 0;
    for y in 0..space[0].len() {
        let mut empty = true;
        for x in 0..space.len() {
            if space[x][y] == '#' {
                empty = false;
            }
        }
        if empty {
            for g in &mut galaxies {
                if g.1 > real_y {
                    g.1 += expansion;
                }
            }
            real_y += expansion;
        }
        real_y += 1;
    }

    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i == j {
                continue;
            }
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let distance = (g1.0 as i64 - g2.0 as i64).abs() + (g1.1 as i64 - g2.1 as i64).abs();
            // println!("{:?} -> {:?}: {}", g1, g2, distance);
            distances.push(distance);
        }
    }
    let acc = distances.iter().sum::<i64>();
    println!("{}", acc);
}
