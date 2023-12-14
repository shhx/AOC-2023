use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
};

type Pos = (usize, usize);

fn inclinate(
    circles: &mut Vec<Pos>,
    squares: &Vec<Pos>,
    dir: (isize, isize),
    bounds: (usize, usize),
) {
    assert!(circles.len() > 0);
    let mut index = 0;
    let mut x = circles[0].0 as isize;
    let mut y = circles[0].1 as isize;
    while index < circles.len() {
        assert!(x >= 0);
        assert!(y >= 0);
        circles[index] = (x as usize, y as usize);
        x += dir.0;
        y += dir.1;
        if x < 0
            || y < 0
            || x >= bounds.0 as isize
            || y >= bounds.1 as isize
            || squares.contains(&(x as usize, y as usize))
            || circles.contains(&(x as usize, y as usize))
        {
            index += 1;
            if index < circles.len() {
                x = circles[index].0 as isize;
                y = circles[index].1 as isize;
            }
        }
    }
}

fn print_map(circles: &Vec<Pos>, squares: &Vec<Pos>, x_len: usize, y_len: usize) {
    println!("------------");
    let mut map = vec![vec!['.'; y_len]; x_len];
    for (x, y) in circles {
        map[*x][*y] = 'O';
    }
    for (x, y) in squares {
        map[*x][*y] = '#';
    }
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
    println!("------------");
}

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let map = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut circles = Vec::new();
    let mut squares = Vec::new();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 'O' {
                circles.push((x, y));
            } else if map[x][y] == '#' {
                squares.push((x, y));
            }
        }
    }
    let mut circles2 = circles.clone();
    let x_len = map.len();
    let y_len = map[0].len();
    let bounds = (x_len, y_len);
    circles.sort_by_key(|(x, _)| *x);
    inclinate(&mut circles, &squares, (-1, 0), bounds);
    // print_map(&circles, &squares, map.len(), map[0].len());
    let north_weight = circles.iter().map(|(x, _)| x_len - x).sum::<usize>();
    println!("{}", north_weight);
    // Part 2
    let mut cache = HashMap::new();
    let loops = 1000000000;
    let mut loop1 = -1;
    let mut loop2 = -1;
    for i in 0..loops {
        circles2.sort_by_key(|(x, _)| *x);
        inclinate(&mut circles2, &squares, (-1, 0), bounds);
        // println!("North");
        // print_map(&circles2, &squares, x_len, y_len);
        circles2.sort_by_key(|(_, y)| *y);
        inclinate(&mut circles2, &squares, (0, -1), bounds);
        // println!("West");
        // print_map(&circles2, &squares, x_len, y_len);
        circles2.sort_by_key(|(x, _)| *x);
        circles2.reverse();
        inclinate(&mut circles2, &squares, (1, 0), bounds);
        // println!("South");
        // print_map(&circles2, &squares, x_len, y_len);
        circles2.sort_by_key(|(_, y)| *y);
        circles2.reverse();
        inclinate(&mut circles2, &squares, (0, 1), bounds);
        // println!("East");
        // print_map(&circles2, &squares, x_len, y_len);
        circles2.sort();
        if cache.contains_key(&circles2) {
            if loop1 == -1 {
                loop1 = cache[&circles2];
                loop2 = i;
                // println!("Loop: {}->{}", loop1, loop2);
                break;
            }
        } else {
            cache.insert(circles2.clone(), i);
        }
    }
    let pos = (loops - loop1) % (loop2 - loop1) + loop1 - 1;
    // println!("Pos: {}", pos);
    let circles = cache.iter().find(|(_, &v)| v == pos).unwrap().0;
    let north_weight = circles.iter().map(|(x, _)| x_len - x).sum::<usize>();
    println!("{}", north_weight);
}
