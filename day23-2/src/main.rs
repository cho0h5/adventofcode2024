use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Graph(HashMap<String, HashSet<String>>);

fn input() -> Graph {
    let mut map = HashMap::new();

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        let (part1, part2) = line.split_once('-').unwrap();

        let inner_map =
            map.entry(part1.to_owned()).or_insert_with(HashSet::new);
        inner_map.insert(part2.to_owned());

        let inner_map =
            map.entry(part2.to_owned()).or_insert_with(HashSet::new);
        inner_map.insert(part1.to_owned());
    }

    Graph(map)
}

impl Graph {
    fn print(&self) {
        for (k, v) in &self.0 {
            println!("{}: {:?}", k, v);
        }
    }
}

fn main() {
    let graph = input();
    graph.print();
}
