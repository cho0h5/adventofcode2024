use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

type Map = HashMap<char, Vec<(isize, isize)>>;
type AntiMap = Vec<Vec<bool>>;

fn input() -> (Map, usize, usize) {
    let mut map: Map = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    let filename = env::args().nth(1).unwrap();
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        width += 1;
        height = line.len();
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            map.entry(c).or_default().push((i as isize, j as isize));
        }
    }

    (map, width, height)
}

fn print_map(map: &Map) {
    for (k, v) in map {
        println!("{}: {:?}", k, v);
    }
}

fn find_antinodes(antimap: &mut AntiMap, antennas: &[(isize, isize)]) {
    let mut set_antinode = |x: isize, y: isize| {
        if x >= 0
            && x < antimap.len() as isize
            && y >= 0
            && y < antimap[0].len() as isize
        {
            antimap[x as usize][y as usize] = true;
        }
    };

    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i == j {
                continue;
            }

            let dx = antennas[j].0 - antennas[i].0;
            let dy = antennas[j].1 - antennas[i].1;
            let nx = antennas[j].0 + dx;
            let ny = antennas[j].1 + dy;
            set_antinode(nx, ny);
        }
    }
}

fn print_antimap(antimap: &AntiMap) {
    for line in antimap {
        println!("{:?}", line);
    }
}

fn main() {
    let (map, width, height) = input();
    let mut antimap: AntiMap = vec![vec![false; width]; height];
    print_map(&map);

    for antennas in map.values() {
        find_antinodes(&mut antimap, antennas);
    }
    print_antimap(&antimap);

    let count = antimap
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x)
        .count();
    println!("antinodes: {}", count);
}
