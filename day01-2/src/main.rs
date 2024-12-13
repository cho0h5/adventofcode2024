use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let filename = "input";
    let mut left = vec![];
    let mut right = HashMap::new();

    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let left_num = numbers[0].parse::<i32>().unwrap();
        let right_num = numbers[1].parse::<i32>().unwrap();

        left.push(left_num);
        let count = right.entry(right_num).or_insert(0);
        *count += 1;
    }

    let mut similarity_score = 0;
    for left_num in left {
        similarity_score += left_num * right.get(&left_num).unwrap_or(&0);
    }
    println!("{}", similarity_score);
}
