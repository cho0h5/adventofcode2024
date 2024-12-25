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

fn bron_kerbosch(
    graph: &Graph,
    r: &mut Vec<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
) -> Vec<Vec<String>> {
    if p.is_empty() && x.is_empty() {
        return vec![r.clone()];
    }

    let mut cliques = Vec::new();
    for v in p.clone() {
        r.push(v.clone());
        let mut p_intersection =
            p.intersection(&graph.0[&v]).cloned().collect();
        let mut x_intersection =
            x.intersection(&graph.0[&v]).cloned().collect();

        cliques.extend(bron_kerbosch(
            graph,
            r,
            &mut p_intersection,
            &mut x_intersection,
        ));

        r.pop();
        p.remove(&v);
        x.insert(v);
    }
    cliques
}

fn main() {
    let graph = input();
    graph.print();

    let mut r = Vec::new();
    let mut p = graph.0.keys().cloned().collect::<HashSet<_>>();
    let mut x = HashSet::new();
    let cliques = bron_kerbosch(&graph, &mut r, &mut p, &mut x);

    let mut maximum_clique =
        cliques.into_iter().max_by_key(|c| c.len()).unwrap();
    maximum_clique.sort();
    println!("maximum clique: {}", maximum_clique.join(","));
}
