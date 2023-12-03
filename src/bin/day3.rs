use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let string = read_to_string("input3.txt").expect("Error reading file");
    let lines = string.lines().collect::<Vec<&str>>();
    let b = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut nums: HashMap<(usize, usize), usize> = HashMap::new();
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut acc = 0;
    let mut i = 0;
    while i < b.len() {
        let mut j = 0;
        while j < b[i].len() {
            if b[i][j].is_digit(10) {
                let mut num = String::new();
                while j < b[i].len() && b[i][j].is_digit(10) {
                    num.push(b[i][j]);
                    j += 1;
                }
                nums.insert((i, j - num.len()), num.parse::<usize>().unwrap());
            }
            j += 1;
        }
        i += 1;
    }
    for ((i, j), num) in nums.iter() {
        let mut valid = false;
        for y in -1..=num.to_string().len() as i32 {
            if valid {
                break;
            }
            for x in -1..=1 {
                let new_i = *i as i32 + x;
                let new_j = *j as i32 + y;
                if new_i >= 0 && new_j >= 0 && new_i < b.len() as i32 && new_j < b[0].len() as i32 {
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;
                    if b[new_i][new_j] == '.' {
                        continue;
                    }
                    // Get gears
                    if b[new_i][new_j] == '*' {
                        gears.entry((new_i, new_j)).or_insert(Vec::new()).push(*num);
                    }
                    if b[new_i][new_j].is_ascii() && !b[new_i][new_j].is_digit(10) {
                        // println!("Sym ({}, {}): {}", new_i, new_j, b[new_i][new_j]);
                        valid = true;
                        break;
                    }
                }
            }
        }
        if valid {
            acc += num;
        }
    }
    let acc2 = gears
        .values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.iter().fold(1, |acc, x| acc * x))
        .sum::<usize>();

    println!("{:?}", acc);
    println!("{:?}", acc2);
}
