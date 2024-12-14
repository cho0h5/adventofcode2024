use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

fn input() -> (Vec<Vec<i32>>, Vec<(i32, i32)>) {
    let mut map = vec![];
    let mut start_points = vec![];

    let filename = env::args().nth(1).unwrap();
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        let mut temp = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    temp.push(-1);
                }
                '0' => {
                    start_points.push((i as i32, j as i32));
                    temp.push(c as i32 - '0' as i32);
                }
                c => {
                    temp.push(c as i32 - '0' as i32);
                }
            }
        }
        map.push(temp);
    }

    (map, start_points)
}

fn print_map(map: &[Vec<i32>]) {
    println!("map:");
    for line in map {
        println!("{:?}", line);
    }
    println!();
}

fn bfs(map: &[Vec<i32>], start: (i32, i32)) -> usize {
    let width = map.len() as i32;
    let height = map[0].len() as i32;
    let mut score = 0;
    let mut deque = VecDeque::new();
    deque.push_back((start.0, start.1, 0));

    while let Some((cx, cy, cs)) = deque.pop_front() {
        let dx = [0, 1, 0, -1];
        let dy = [1, 0, -1, 0];
        if cs == 9 {
            score += 1;
            continue;
        }

        for i in 0..4 {
            let nx = cx + dx[i];
            let ny = cy + dy[i];
            let ns = cs + 1;
            if nx < 0 || nx >= width || ny < 0 || ny >= height {
                continue;
            }
            if ns != map[nx as usize][ny as usize] {
                continue;
            }
            deque.push_back((nx, ny, ns));
        }
    }
    score
}

fn main() {
    let (map, start_points) = input();
    println!("start points: {:?}", start_points);
    print_map(&map);

    let mut total_score = 0;
    for start in start_points {
        let score = bfs(&map, start);
        println!("score: {}", score);
        total_score += score;
    }
    println!("total score: {}", total_score);
}
