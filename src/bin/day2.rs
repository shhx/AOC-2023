use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let string = read_to_string("input2.txt").expect("Error reading file");
    let max_values: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut acc = 0;
    let mut acc2 = 0;
    for line in string.lines() {
        let (game, sets) = line.split_once(":").expect("Error splitting line");
        let game_index = game.split_at(5).1.parse::<usize>().expect("Error parsing game index");
        let mut valid = true;
        let mut max_needed_values = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for set in sets.split(";") {
            for game in set.trim().split(",") {
                let (number, color) = game.trim().split_once(" ").expect("Error splitting game");
                let number = number.parse::<usize>().expect("Error parsing number");
                if number > *max_needed_values.get(color).unwrap() {
                    max_needed_values.insert(color, number);
                }
                if number > *max_values.get(color).unwrap() {
                    valid = false;
                }
            }
        }
        acc2 += max_needed_values.values().fold(1, |acc, x| acc * x);
        if valid {
            acc += game_index;
        }
    }
    println!("{acc}");
    println!("{acc2}");
}
