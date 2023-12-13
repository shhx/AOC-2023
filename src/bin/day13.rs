use std::{collections::HashMap, env::args, fs::read_to_string};


#[derive(Debug)]
enum Found {
    No,
    Yes(usize),
}
fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let maps = string.split("\n\n").collect::<Vec<_>>();
    let mut acc = 0;
    let mut acc2 = 0;
    for map in maps {
        let map = map
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut mirror_x = Found::No;
        let mut mirror_y = Found::No;
        let mut fixable_x = Found::No;
        let mut fixable_y = Found::No;
        let x_len = map.len();
        let y_len = map[0].len();
        for x in 0..x_len-1 {
            let mut errors = 0;
            for y in 0..y_len {
                let mut index = 0;
                while index <= x && (x + index + 1) < x_len {
                    errors += (map[x - index][y] != map[x + index + 1][y]) as usize;
                    index += 1;
                }
            }
            match errors {
                0 => mirror_x = Found::Yes(x + 1),
                1 => fixable_x = Found::Yes(x + 1),
                _ => (),
            }
        }
        for y in 0..y_len-1 {
            let mut errors = 0;
            for x in 0..x_len {
                let mut index = 0;
                while index <= y && (y + index + 1) < y_len {
                    errors += (map[x][y - index] != map[x][y + index + 1]) as usize;
                    index += 1;
                }
            }
            match errors {
                0 => mirror_y = Found::Yes(y + 1),
                1 => fixable_y = Found::Yes(y + 1),
                _ => (),
            }
        }
        // println!("------------");
        // println!("{:?} {:?}", mirror_x, mirror_y);
        // println!("Fixable: {:?} {:?}", fixable_x, fixable_y);

        if let Found::Yes(x) = mirror_x {
            acc += 100 * x;
        }
        if let Found::Yes(y) = mirror_y {
            acc +=  y;
        }
        if let Found::Yes(x) = fixable_x {
            acc2 += 100 * x;
        }
        if let Found::Yes(y) = fixable_y {
            acc2 += y;
        }
    }
    println!("{}", acc);
    println!("{}", acc2);
}
