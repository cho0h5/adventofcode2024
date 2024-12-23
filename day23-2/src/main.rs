use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

type Graph = HashMap<String, HashSet<String>>;

fn input() -> Graph {
    let mut map: Graph = HashMap::new();

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

    map
}

fn bf(graph: &Graph, ts: &HashSet<String>) -> HashSet<Vec<String>> {
    let mut inter_connected = HashSet::new();
    for t in ts {
        let t_map = &graph[t];

        println!("{}, {:?}", t, t_map);

        for u in t_map {
            if let Some(u_map) = graph.get(u) {
                for v in u_map {
                    if t_map.contains(v) {
                        let mut key = vec![t.clone(), u.clone(), v.clone()];
                        key.sort();
                        inter_connected.insert(key);
                    }
                }
            }
        }
    }

    inter_connected
}

fn main() {
    let graph = input();
    println!("{:?}", graph);
}
