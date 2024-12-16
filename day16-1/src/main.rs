use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::read_to_string;

fn input() -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let mut map = vec![];
    let mut s_pos = (0, 0);
    let mut e_pos = (0, 0);

    let filename = env::args().nth(1).unwrap();
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        let mut temp = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    s_pos = (i as i32, j as i32);
                    temp.push('.');
                }
                'E' => {
                    e_pos = (i as i32, j as i32);
                    temp.push('.');
                }
                c => {
                    temp.push(c);
                }
            }
        }
        map.push(temp);
    }

    (map, s_pos, e_pos)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    x: i32,
    y: i32,
    dir: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn print_map(map: &[Vec<char>]) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn bfs(map: &mut [Vec<char>], s_pos: (i32, i32), e_pos: (i32, i32)) -> i32 {
    let width = map.len();
    let height = map[0].len();
    let mut vst = vec![vec![[-1; 4]; height]; width];
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        x: s_pos.0,
        y: s_pos.1,
        dir: 0,
    });

    while let Some(State {
        cost: cc,
        x: cx,
        y: cy,
        dir: cd,
    }) = heap.pop()
    {
        if vst[cx as usize][cy as usize][cd as usize] != -1 {
            continue;
        }
        vst[cx as usize][cy as usize][cd as usize] = cc;
        println!("----------------");
        println!("search: {}, ({}, {}), {}", cc, cx, cy, cd);
        if cx == e_pos.0 && cy == e_pos.1 {
            return cc;
        }

        for i in 0..1 {
            let dx = [0, 1, 0, -1];
            let dy = [1, 0, -1, 0];

            let nc = cc + 1;
            let nx = cx + dx[cd as usize];
            let ny = cy + dy[cd as usize];
            let nd = cd;

            if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                continue;
            }

            if map[nx as usize][ny as usize] == '#' {
                continue;
            }

            if vst[nx as usize][ny as usize][nd as usize] != -1 {
                continue;
            }

            match nd {
                0 => map[nx as usize][ny as usize] = '>',
                1 => map[nx as usize][ny as usize] = 'v',
                2 => map[nx as usize][ny as usize] = '<',
                3 => map[nx as usize][ny as usize] = '^',
                _ => (),
            }
            println!("push: {}, ({}, {}), {}", nc, nx, ny, nd);
            heap.push(State {
                cost: nc,
                x: nx,
                y: ny,
                dir: nd,
            });
        }

        for i in 1..4 {
            let nc = if i == 2 { cc + 2000 } else { cc + 1000 };
            let nx = cx;
            let ny = cy;
            let nd = (cd + i) % 4;
            if vst[nx as usize][ny as usize][nd as usize] == -1 {
                println!("push: {}, ({}, {}), {}", nc, nx, ny, nd);
                heap.push(State {
                    cost: nc,
                    x: nx,
                    y: ny,
                    dir: nd,
                });
            }
        }
    }
    -1
}

fn main() {
    let (mut map, s_pos, e_pos) = input();
    print_map(&map);
    println!("s_pos: {:?}", s_pos);
    println!("e_pos: {:?}", e_pos);
    let cost = bfs(&mut map, s_pos, e_pos);
    print_map(&map);
    println!("cost: {}", cost);
}
