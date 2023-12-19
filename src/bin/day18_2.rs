use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let dirs: HashMap<_, _> = HashMap::from([
        (3, (-1, 0)),
        (1, (1, 0)),
        (2, (0, -1)),
        (0, (0, 1)),
    ]);

    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let mut current_pos = (0, 0);
    let mut vertex = Vec::new();
    let mut border = 0;
    for line in lines {
        let color = line.split_once("#").unwrap().1;
        let color = color.split_once(")").unwrap().0;
        let amount = color.chars().take(5).join("");
        let amount = isize::from_str_radix(&amount, 16).unwrap();
        let dir = color.chars().last().unwrap().to_digit(10).unwrap() as usize;
        let dir_val = dirs[&dir];

        border += amount;
        current_pos = (current_pos.0 + dir_val.0 * amount, current_pos.1 + dir_val.1 * amount);
        vertex.push(current_pos);
    }

    // Shoe lace formula
    let mut area = 0;
    for (v1, v2) in vertex.iter().zip(vertex.iter().skip(1)) {
        let (y1, x1) = v1;
        let (y2, x2) = v2;
        area += x1*y2 - x2*y1;
    }
    area = area / 2;

    // Pick's theorem
    // A = i + b/2 - 1
    // i = A - b/2 + 1
    // i + b = A - b/2 + 1 + b
    // i + b = A + b/2 + 1
    println!("Border: {}", border);
    println!("Area: {}", area + border/2 + 1);
}
