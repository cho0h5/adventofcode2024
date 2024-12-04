use std::fs::read_to_string;

fn is_increasing(numbers: &[i32], thresh: i32) -> bool {
    fn is_valid(gap: i32) -> bool {
        1 <= gap && gap <= 3
    }

    let mut count = 0;

    for i in 1..numbers.len() {
        if !is_valid(numbers[i] - numbers[i - 1]) {
            if i >= 2 && is_valid(numbers[i] - numbers[i - 2]) {
                count += 1;
            } else {
                return false;
            }
        }
    }
    count <= thresh
}

fn is_decreasing(numbers: &[i32], thresh: i32) -> bool {
    fn is_valid(gap: i32) -> bool {
        -3 <= gap && gap <= -1
    }

    let mut count = 0;

    for i in 1..numbers.len() {
        if !is_valid(numbers[i] - numbers[i - 1]) {
            if i >= 2 && is_valid(numbers[i] - numbers[i - 2]) {
                count += 1;
            } else {
                return false;
            }
        }
    }

    count <= thresh
}

fn main() {
    let filename = "input";

    let mut count = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();

        if is_increasing(&numbers, 1)
            || is_decreasing(&numbers, 1)
            || is_increasing(&numbers[1..], 0)
            || is_decreasing(&numbers[1..], 0) {
            count += 1;
            println!("Safe");
        } else {
            println!("Unsafe");
        }
    }
    println!("{}", count);
}

