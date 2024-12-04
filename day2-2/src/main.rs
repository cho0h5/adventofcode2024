use std::fs::read_to_string;

fn is_increasing(numbers: &Vec<i32>) -> bool {
    fn is_valid(gap: i32) -> bool {
        1 <= gap && gap <= 3
    }

    let mut count = 0;

    for i in 1..numbers.len() {
        if !is_valid(numbers[i] - numbers[i - 1]) {
            return false;
        }
    }
    return true;
}

fn is_decreasing(numbers: &Vec<i32>) -> bool {
    fn is_valid(gap: i32) -> bool {
        -3 <= gap && gap <= -1
    }

    let mut count = 0;

    for i in 1..numbers.len() {
        if !is_valid(numbers[i] - numbers[i - 1]) {
            return false;
        }
    }
    return true;
}

fn main() {
    let filename = "input";

    let mut count = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();

        if is_increasing(&numbers) {
            count += 1;
            println!("Safe");
        } else if is_decreasing(&numbers) {
            count += 1;
            println!("Safe");
        } else {
            println!("Unsafe");
        }
    }
    println!("{}", count);
}

