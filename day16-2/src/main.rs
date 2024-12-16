use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
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

fn print_vst(vst0: &[Vec<[i32; 4]>]) {
    for line in vst0 {
        for c in line {
            print!("({:4}, {:4}, {:4}, {:4}) ", c[0], c[1], c[2], c[3]);
        }
        println!();
    }
}

fn print_vst2(vst2: &[Vec<i32>]) {
    for line in vst2 {
        for c in line {
            if *c == -1 {
                print!("    ");
            } else {
                print!("{:4}", c);
            }
        }
        println!();
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

fn bfs(
    map: &mut [Vec<char>],
    s_pos: (i32, i32),
    e_pos: (i32, i32),
) -> (i32, Vec<Vec<[i32; 4]>>) {
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
        // println!("----------------");
        // println!("search: {}, ({}, {}), {}, {}", cc, cx, cy, cd, cc2);
        if cx == e_pos.0 && cy == e_pos.1 {
            return (cc, vst);
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
            // println!("push: {}, ({}, {}), {}, {}", nc, nx, ny, nd, nc2);
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
                // println!("push: {}, ({}, {}), {}, {}", nc, nx, ny, nd, nc2);
                heap.push(State {
                    cost: nc,
                    x: nx,
                    y: ny,
                    dir: nd,
                });
            }
        }
    }
    (-1, vst)
}

fn bfs_vst(vst: &[Vec<[i32; 4]>], e_pos: (i32, i32)) -> (i32, Vec<Vec<i32>>) {
    let width = vst.len();
    let height = vst[0].len();
    let mut vst2 = vec![vec![-1; height]; width];
    let mut seats = 0;
    let mut deque = VecDeque::new();

    let cx = e_pos.0;
    let cy = e_pos.1;
    let cc = vst[cx as usize][cy as usize];
    deque.push_back((cx, cy, cc, 0));

    while let Some((cx, cy, cc, cc2)) = deque.pop_front() {
        if vst2[cx as usize][cy as usize] != -1 {
            continue;
        }
        vst2[cx as usize][cy as usize] = cc2;
        seats += 1;

        let dx = [0, 1, 0, -1];
        let dy = [1, 0, -1, 0];

        for i in 0..4 {
            let nx = cx + dx[i];
            let ny = cy + dy[i];
            let nc2 = cc2 + 1;
            if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                continue;
            }
            if vst2[nx as usize][ny as usize] != -1 {
                continue;
            }

            let nc = vst[nx as usize][ny as usize];
            for j in 0..4 {
                if nc[j] != -1 && cc[j] - nc[j] == 1 {
                    deque.push_back((nx, ny, nc, nc2));
                    break;
                }
            }
        }
    }

    (seats, vst2)
}

fn main() {
    let (mut map, s_pos, e_pos) = input();
    print_map(&map);
    println!("s_pos: {:?}", s_pos);
    println!("e_pos: {:?}", e_pos);

    let (cost, vst) = bfs(&mut map, s_pos, e_pos);
    println!("cost: {:?}", cost);
    print_map(&map);
    print_vst(&vst);

    let (seats, vst2) = bfs_vst(&vst, e_pos);
    println!("seats: {:?}", seats);
    print_vst2(&vst2);
}
