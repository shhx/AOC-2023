use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

fn main() {
    let string = read_to_string("input4.txt").expect("Error reading file");
    let lines = string.lines().collect::<Vec<&str>>();
    let mut acc = 0;
    let mut map = HashMap::new();
    let mut all_cards = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let (_, cards) = line.split_once(": ").unwrap();
        let (win, mine) = cards.split_once(" | ").unwrap();
        let win = win.trim()
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();
        let mine = mine.trim()
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
        let same = win.intersection(&mine).collect::<Vec<&usize>>();
        let points;
        if same.len() > 0 {
            points = 2_usize.pow(same.len() as u32 - 1);
        } else {
            points = 0;
        }
        acc += points;
        map.insert(i+1, (i+2..=i+same.len()+1).collect::<Vec<usize>>());
        all_cards.insert(i+1, 1);
    }

    for i in 1..lines.len()+1 {
        let a = all_cards.get(&i).unwrap().to_owned();
        for j in map.get(&i).unwrap() {
            *all_cards.entry(*j).or_insert(0) += a;
        }
    }

    println!("{}", acc);
    println!("{}", all_cards.values().fold(0, |acc, x| acc + x));
}
