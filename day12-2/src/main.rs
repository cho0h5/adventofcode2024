use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

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

#[allow(dead_code)]
fn print_border(map: &Vec<Vec<[bool; 4]>>) {
    println!("border:");
    for line in map {
        println!("{:?}", line);
    }
    println!();
}

fn bfs(
    map: &[Vec<char>],
    vst: &mut [Vec<bool>],
    x: isize,
    y: isize,
) -> (isize, Vec<Vec<[bool; 4]>>) {
    let id = map[x as usize][y as usize];
    let mut deque = VecDeque::from(vec![(x, y)]);
    let mut area = 0;
    let border_rows = map.len() + 2;
    let border_cols = map[0].len() + 2;
    let mut border = vec![vec![[false; 4]; border_cols]; border_rows];

    while let Some((cx, cy)) = deque.pop_front() {
        if vst[cx as usize][cy as usize] {
            continue;
        }
        vst[cx as usize][cy as usize] = true;
        area += 1;

        let dl = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dl {
            let nx = cx + dx;
            let ny = cy + dy;

            if nx >= 0
                && nx < map.len() as isize
                && ny >= 0
                && ny < map[0].len() as isize
                && !vst[nx as usize][ny as usize]
                && map[nx as usize][ny as usize] == id
            {
                deque.push_back((nx, ny));
            } else {
                if nx >= 0
                    && nx < map.len() as isize
                    && ny >= 0
                    && ny < map[0].len() as isize
                    && map[nx as usize][ny as usize] == id
                {
                    continue;
                }

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
    (area, border)
}

fn perimeter_each(
    border: &[Vec<[bool; 4]>],
    index: usize,
    vst: &mut [Vec<bool>],
    i: isize,
    j: isize,
) {
    let mut deque = VecDeque::new();
    deque.push_back((i, j));

    while let Some((cx, cy)) = deque.pop_front() {
        if vst[cx as usize][cy as usize] {
            continue;
        }
        vst[cx as usize][cy as usize] = true;

        let dl = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dl {
            let nx = cx + dx;
            let ny = cy + dy;

            if nx >= 0
                && nx < border.len() as isize
                && ny >= 0
                && ny < border[0].len() as isize
                && !vst[nx as usize][ny as usize]
                && border[nx as usize][ny as usize][index]
            {
                deque.push_back((nx, ny));
            }
        }
    }
}

fn count_perimeter(border: &[Vec<[bool; 4]>], index: usize) -> isize {
    let mut vst: Vec<Vec<bool>> =
        border.iter().map(|row| vec![false; row.len()]).collect();
    let mut count = 0;

    for i in 0..border.len() {
        for j in 0..border[0].len() {
            if !border[i][j][index] {
                continue;
            }
            if vst[i][j] {
                continue;
            }

            perimeter_each(border, index, &mut vst, i as isize, j as isize);
            count += 1;
        }
    }
    count
}

fn bfs_border(border: &[Vec<[bool; 4]>]) -> isize {
    let mut perimeter = 0;

    perimeter += count_perimeter(border, 0);
    perimeter += count_perimeter(border, 1);
    perimeter += count_perimeter(border, 2);
    perimeter += count_perimeter(border, 3);
    perimeter
}

fn main() {
    let map = input();
    print_map(&map);
    let mut vst: Vec<Vec<bool>> =
        map.iter().map(|row| vec![false; row.len()]).collect();

    let mut total_price = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                continue;
            }

            let (area, border) = bfs(&map, &mut vst, i as isize, j as isize);
            if area != 0 {
                let perimeter = bfs_border(&border);
                println!(
                    "{}, {}, {}: {}, {}",
                    i, j, map[i][j], area, perimeter
                );
                total_price += area * perimeter;
            }
        }
    }
    println!("total_price: {}", total_price);
}
