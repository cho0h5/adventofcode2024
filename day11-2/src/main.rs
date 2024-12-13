use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn input() -> HashMap<i64, i64> {
    let mut stones = HashMap::new();

    let filename = env::args().nth(1).unwrap();
    let binding = read_to_string(filename).unwrap();
    let line = binding.lines().next().unwrap();
    for number in line.split_whitespace() {
        *stones.entry(number.parse::<i64>().unwrap()).or_insert(0) += 1;
    }

    stones
}

fn blink(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut results = HashMap::new();

    for (stone, count) in stones {
        let stone_str = stone.to_string();
        let stone_len = stone_str.len();
        if stone == 0 {
            *results.entry(1).or_insert(0) += count;
        } else if stone_len % 2 == 0 {
            let num1 = stone_str[..stone_len / 2].parse::<i64>().unwrap();
            let num2 = stone_str[stone_len / 2..].parse::<i64>().unwrap();
            *results.entry(num1).or_insert(0) += count;
            *results.entry(num2).or_insert(0) += count;
        } else {
            *results.entry(stone * 2024).or_insert(0) += count;
        }
    }

    results
}

fn count(stones: &HashMap<i64, i64>) -> i64 {
    let mut sum = 0;

    for count in stones.values() {
        sum += count;
    }

    sum
}

fn main() {
    let mut stones = input();
    println!("init: {:#?}", stones);
    println!("{}: {:?}", 0, stones.len());

    for i in 1..=75 {
        stones = blink(stones);
        println!("{}: {:?}", i, count(&stones));
    }
}
