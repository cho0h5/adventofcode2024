use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

struct Map {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    cost: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new() -> Map {
        let mut map = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        let filename = env::args().nth(1).unwrap();
        for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
            let mut temp = Vec::new();
            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (i, j);
                } else if c == 'E' {
                    end = (i, j);
                }
                temp.push(c);
            }
            map.push(temp);
        }

        let width = map.len();
        let height = map[0].len();
        let cost = vec![vec![usize::MAX; height]; width];

        Map {
            map,
            start,
            end,
            cost,
            width,
            height,
        }
    }

    fn bfs_cost(&mut self) {
        let mut deque =
            VecDeque::from(vec![(self.end.0 as i32, self.end.1 as i32, 0)]);

        while let Some((cx, cy, cc)) = deque.pop_front() {
            if cc >= self.cost[cx as usize][cy as usize] {
                continue;
            }
            self.cost[cx as usize][cy as usize] = cc;

            for i in 0..4 {
                let dx = [0, 1, 0, -1];
                let dy = [1, 0, -1, 0];

                let nx = cx + dx[i];
                let ny = cy + dy[i];
                let nc = cc + 1;
                if nx < 0
                    || nx >= self.width as i32
                    || ny < 0
                    || ny >= self.height as i32
                {
                    continue;
                }
                if nc >= self.cost[nx as usize][ny as usize] {
                    continue;
                }
                if self.map[nx as usize][ny as usize] == '#' {
                    continue;
                }

                deque.push_back((nx, ny, nc));
            }
        }
    }

    fn print(&self) {
        for (i, line) in self.map.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if (i, j) == self.start {
                    print!("S");
                } else if (i, j) == self.end {
                    print!("E");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }

        for line in &self.cost {
            for c in line {
                if *c == usize::MAX {
                    print!("{:3}", " ");
                } else {
                    print!("{:3}", c);
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut map = Map::new();
    map.bfs_cost();
    map.print();
}
