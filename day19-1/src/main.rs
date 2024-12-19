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

fn main() {
    let (availables, wanteds) = input();
    println!("{:?}", availables);
    println!("{:?}", wanteds);
}
