use std::env;
use std::collections::HashMap;
use std::fs::read_to_string;

fn input() -> (HashMap<String, bool>) {
    let mut wires = HashMap::new();

    let mut flag = false;
    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            flag = true;
        }

        if !flag {
            let (wire, value) = line.split_once(": ").unwrap();
            let value: usize = value.parse().unwrap();

            match value {
                0 => wires.insert(wire.to_owned(), false),
                1 => wires.insert(wire.to_owned(), true),
                _ => panic!("no way"),
            };
        } else {}
    }

    (wires)
}

fn main() {
    let (wires) = input();
    println!("wires: {:?}", wires);
}