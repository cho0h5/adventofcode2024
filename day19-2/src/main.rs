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

fn is_possible(
    availables: &[String],
    wanted: &str,
    pos: usize,
    dp: &mut [usize],
) {
    if pos > wanted.len() {
        return;
    }

    if dp[pos] > 0 {
        for avail in availables {
            if wanted[pos..].starts_with(avail) {
                dp[pos + avail.len()] += dp[pos];
            }
        }
    }

    is_possible(availables, wanted, pos + 1, dp);
}

fn main() {
    let (availables, wanteds) = input();
    println!("{:?}", availables);
    // println!("{:?}", wanteds);

    let mut count = 0;
    for wanted in wanteds {
        let mut dp = vec![0; wanted.len() + 1];
        dp[0] = 1;
        is_possible(&availables, &wanted, 0, &mut dp);
        println!("{} <- {}", dp[wanted.len()], wanted);
        println!("dp: {:?}", dp);
        count += dp[wanted.len()];
    }
    println!("count: {}", count);
}
