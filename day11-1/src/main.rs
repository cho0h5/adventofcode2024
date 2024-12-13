use std::env;
use std::fs::read_to_string;

fn input() -> Vec<String> {
    let mut stones = vec![];

    let filename = env::args().nth(1).unwrap();
    let binding = read_to_string(filename).unwrap();
    let line = binding.lines().next().unwrap();
    for number in line.split_whitespace() {
        stones.push(number.to_string());
    }

    stones
}

fn blink(stones: Vec<String>) -> Vec<String> {
    let mut results = vec![];

    for stone in stones {
        let number = stone.parse::<i64>().unwrap();
        match stone.len() {
            1 if number == 0 => {
                results.push(1.to_string());
            }
            x if x % 2 == 0 => {
                let num1 = stone[..x / 2].to_owned().parse::<i64>().unwrap();
                let num2 = stone[x / 2..].to_owned().parse::<i64>().unwrap();
                results.push(num1.to_string());
                results.push(num2.to_string());
            }
            _ => {
                results.push((number * 2024).to_string());
            }
        }
    }

    results
}

fn main() {
    let mut stones = input();
    println!("init: {:?}", stones);
    println!("{}: {:?}", 0, stones.len());

    for i in 1..=25 {
        stones = blink(stones);
        // println!("{}: {:?}", i, stones);
        println!("{}: {:?}", i, stones.len());
    }
}
