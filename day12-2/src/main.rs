use std::env;
use std::fs::read_to_string;
use std::collections::VecDeque;

fn input() -> Vec<Vec<char>> {
    let mut map = vec![];

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        map.push(line.chars().collect());
    }

    map
}

fn print_map(map: &Vec<Vec<char>>) {
    println!("map:");
    for line in map {
        println!("{:?}", line);
    }
    println!();
}

fn print_border(map: &Vec<Vec<[bool; 4]>>) {
    println!("map:");
    for line in map {
        println!("{:?}", line);
    }
    println!();
}

fn bfs(map: &Vec<Vec<char>>, vst: &mut Vec<Vec<bool>>, x: isize, y: isize) -> (isize, isize, Vec<Vec<[bool; 4]>>) {
    let id = map[x as usize][y as usize];
    let mut deque = VecDeque::from(vec![(x, y)]);
    let mut area = 0;
    let mut perimeter = 0;
    let border_rows = map.len() + 2;
    let border_cols = map[0].len() + 2;
    let mut border = vec![vec![[false; 4]; border_cols]; border_rows];

    while let Some((cx, cy)) = deque.pop_front() {
        if vst[cx as usize][cy as usize] {
            continue;
        }
        vst[cx as usize][cy as usize] = true;
        area += 1;
        perimeter += 0;

        let dl = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dl {
            let nx = cx + dx;
            let ny = cy + dy;

            if nx >= 0 && nx < map.len() as isize
                && ny >= 0 && ny < map[0].len() as isize
                && !vst[nx as usize][ny as usize]
                && map[nx as usize][ny as usize] == id {
                deque.push_back((nx, ny));
            } else if (nx >= 0 && nx < map.len() as isize
                && ny >= 0 && ny < map[0].len() as isize
                && map[nx as usize][ny as usize] != id)
            || (nx == 0 || nx == map.len() as isize
                || ny == 0 || ny == map[0].len() as isize) {
                let nnx = (nx + 1) as usize;
                let nny = (ny + 1) as usize;
                match (dx, dy) {
                    (-1, _) => border[nnx][nny][0] = true,
                    (1, _) => border[nnx][nny][1] = true,
                    (_, -1) => border[nnx][nny][2] = true,
                    (_, 1) => border[nnx][nny][3] = true,
                    _ => (),
                }
            }
        }
    }
    (area, perimeter, border)
}

fn main() {
    let map = input();
    print_map(&map);
    let mut vst: Vec<Vec<bool>> = map
        .iter()
        .map(|row| vec![false; row.len()])
        .collect();

    let mut total_price = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                continue;
            }

            let (area, perimeter, border) = bfs(&map, &mut vst, i as isize, j as isize);
            if area != 0 {
                println!("{}, {}: {}, {}", i, j, area, perimeter);
                print_border(&border);
            }
            total_price += area * perimeter;
        }
    }
    println!("total_price: {}", total_price);
}
