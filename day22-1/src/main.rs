use std::env;
use std::fs::read_to_string;

fn input() -> Vec<u64> {
    let mut secrets = Vec::new();
    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        secrets.push(line.parse().unwrap());
    }
    secrets
}

fn next_random(prev: u64) -> u64 {
    let mul64 = prev * 64;
    let step1 = (prev ^ mul64) % 16777216;

    let div32 = ((step1 as f32) / 32.0) as u64;
    let step2 = (step1 ^ div32) % 16777216;

    let mul2048 = step2 * 2048;
    let step3 = (step2 ^ mul2048) % 16777216;

    step3
}

fn main() {
    let mut secrets = input();
    // println!("secrets: {:?}", secrets);

    fn iter_once(secrets: &mut Vec<u64>) {
        for secret in secrets.iter_mut() {
            *secret = next_random(*secret);
        }
        // println!("secrets: {:?}", secrets);
    }

    for _ in 0..2000 {
        iter_once(&mut secrets);
    }

    println!("sum: {}", secrets.iter().sum::<u64>());
}
