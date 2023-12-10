use std::{env::args, fs::read_to_string};

fn main() {
    let file = args().nth(1).expect("Input file name");
    let string = read_to_string(file).expect("Error reading file");
    let lines = string.lines().collect::<Vec<_>>();
    let mut acc = 0;
    let mut acc2 = 0;
    for line in lines {
        let mut nums = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let last_num = nums.last().unwrap();
        let mut diffs = vec![*last_num];
        let mut diffs_back = vec![nums[0]];
        // println!("Orig: {:?}", nums);
        loop {
            // println!("{:?}", nums);
            let len = nums.len();
            let diff = nums[len - 1] - nums[len - 2];
            diffs.push(diff);
            diffs_back.push(nums[1] - nums[0]);
            nums = nums
                .iter()
                .zip(nums.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>();
            if nums.iter().all(|x| *x == 0) {
                break;
            }
        }
        // println!("diffs {:?}", diffs_back);
        let extra = diffs.iter().rev().fold(0, |acc, x| acc + x);
        let extra_back = diffs_back.iter().rev().fold(0, |acc, x| x - acc);
        acc += extra;
        acc2 += extra_back;
        // println!("Extra: {}", extra);
        // println!("Extra back: {}", extra_back);
    }
    println!("{}", acc);
    println!("{}", acc2);
}
