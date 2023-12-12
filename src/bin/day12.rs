use std::{collections::HashMap, env::args, fs::read_to_string};

fn posible_groups(
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
    springs: &Vec<char>,
    groups: &Vec<usize>,
) -> usize {
    if groups.len() == 0 {
        // Some '#' are left and no groups are left
        if springs.iter().filter(|s| **s == '#').count() == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    if springs.len() < groups.iter().sum::<usize>() {
        return 0;
    }
    if springs[0] == '.' {
        return posible_groups(cache, &springs[1..].to_vec(), groups);
    }
    if let Some(&pos) = cache.get(&(springs.clone(), groups.clone())) {
        return pos;
    }

    let mut pos = 0;
    if springs[0] == '?' {
        // There is a '.' instead of '?' in the first position
        pos = posible_groups(cache, &springs[1..].to_vec(), groups);
    }

    if springs.iter().take(groups[0]).all(|&s| s != '.') {
        if springs.len() > groups[0] && (springs.len() == groups[0] || springs[groups[0]] != '#') {
            pos += posible_groups(
                cache,
                &springs[groups[0] + 1..].to_vec(),
                &groups[1..].to_vec(),
            );
        } else if springs.len() == groups[0] {
            pos += posible_groups(cache, &springs[groups[0]..].to_vec(), &groups[1..].to_vec());
        }
    }
    cache.insert((springs.clone(), groups.clone()), pos);
    return pos;
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let mut acc = 0;
    let mut acc2 = 0;
    let mut cache = HashMap::new();
    for line in lines {
        let (springs, groups) = line.split_once(" ").unwrap();
        let groups = groups
            .split(",")
            .map(|g| g.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let springs = springs.chars().collect::<Vec<_>>();
        let springs2 = std::iter::repeat(springs.clone())
            .take(5)
            .map(|mut s| {
                s.push('?');
                s
            })
            .flatten()
            .collect::<Vec<_>>();
        let springs2 = springs2[..springs2.len() - 1].to_vec();
        let groups2 = std::iter::repeat(groups.clone())
            .take(5)
            .flatten()
            .collect::<Vec<_>>();
        // println!("{:?} {:?}", springs2, groups2);
        acc += posible_groups(&mut cache, &springs, &groups);
        acc2 += posible_groups(&mut cache, &springs2, &groups2)
    }
    println!("{}", acc);
    println!("{}", acc2);
}
