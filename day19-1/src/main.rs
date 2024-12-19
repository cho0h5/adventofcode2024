use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

fn input() -> (Vec<String>, Vec<String>) {
    let mut wanteds = Vec::new();

    let filename = env::args().nth(1).unwrap();
    let binding = read_to_string(filename).unwrap();
    let mut iterator = binding.lines();
    let mut availables = iterator
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_owned())
        .collect();
    iterator.next();
    for line in iterator {
        wanteds.push(line.to_owned());
    }
    (availables, wanteds)
}

fn is_possible(availables: &[String], wanted: &str) -> bool {
    let mut dp = vec![0; wanted.len() + 1];

    for _ in 0..60 {
        for avail in availables {
            if wanted.starts_with(avail) {
                dp[avail.len()] += 1;
            }

            for i in 0..wanted.len() {
                if dp[i] != 0 {
                    if wanted[i..].starts_with(avail) {
                        dp[i + avail.len()] = dp[i];
                    }
                }
            }
        }
    }

    dp[wanted.len()] > 0
}

fn main() {
    let (availables, wanteds) = input();
    println!("{:?}", availables);
    // println!("{:?}", wanteds);

    let mut count = 0;
    for wanted in wanteds {
        let is_possible = is_possible(&availables, &wanted);
        println!("{} <- {}", is_possible, wanted);
        if is_possible {
            count += 1;
        }
    }
    println!("count: {}", count);
}
