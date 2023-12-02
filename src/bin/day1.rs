use std::fs::read_to_string;

fn main() {
    let string = read_to_string("input1.txt").expect("Error reading file");
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut acc = 0;
    let mut acc2 = 0;
    for line in string.lines() {
        let mut line_nums: Vec<usize> = Vec::new();
        let mut line_nums2: Vec<usize> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                line_nums.push(c.to_digit(10).unwrap() as usize);
                line_nums2.push(c.to_digit(10).unwrap() as usize);
            }
            for (j, number) in numbers.iter().enumerate() {
                if line[i..].starts_with(number) {
                    line_nums2.push(j + 1);
                }
            }
        }
        let res = line_nums.first().unwrap().to_string() + &line_nums.last().unwrap().to_string();
        let res2 =
            line_nums2.first().unwrap().to_string() + &line_nums2.last().unwrap().to_string();
        acc += res.parse::<usize>().unwrap();
        acc2 += res2.parse::<usize>().unwrap();
    }
    println!("{acc}");
    println!("{acc2}");
}
