use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn input() -> Vec<Vec<u64>> {
    let mut secrets = Vec::new();
    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        secrets.push(vec![line.parse().unwrap()]);
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
    let num_of_monkey = secrets.len();
    let mut prices = vec![Vec::new(); num_of_monkey];
    let mut deltas = vec![Vec::new(); num_of_monkey];
    // println!("secrets: {:?}", secrets);

    for _ in 0..2000 {
        for (j, secret) in secrets.iter_mut().enumerate() {
            let old = *secret.last().unwrap();
            let new = next_random(old);
            secret.push(new);
            prices[j].push(new % 10);
            deltas[j].push(new as i64 % 10 - old as i64 % 10);
        }
    }

    // println!("prices: {:?}", prices);
    // println!("deltas: {:?}", deltas);

    let mut map = Vec::new();
    for i in 0..num_of_monkey {
        let price_len = prices[0].len();
        let mut m = HashMap::new();

        for j in 3..price_len {
            m.entry((
                deltas[i][j - 3],
                deltas[i][j - 2],
                deltas[i][j - 1],
                deltas[i][j - 0],
            ))
            //             .and_modify(|v| {
            //                 if *v < prices[i][j] {
            //                     *v = prices[i][j];
            //                 }
            //             })
            .or_insert(prices[i][j]);
        }
        map.push(m);
    }
    // println!("{:#?}", map);

    let mut merged_map = HashMap::new();
    for m in map {
        for (key, value) in m {
            merged_map
                .entry(key)
                .and_modify(|v: &mut Vec<u64>| v.push(value))
                .or_insert(vec![value]);
        }
    }

    let mut maximum = 0;
    let mut maximum_value = Vec::new();
    let mut maximum_key = (0, 0, 0, 0);
    for (k, v) in merged_map {
        let sum = v.iter().sum();
        if sum > maximum {
            maximum = sum;
            maximum_value = v;
            maximum_key = k;
        }
    }
    println!(
        "maximum: ({:?}, {:?}, {})",
        maximum_key, maximum_value, maximum
    );
}
