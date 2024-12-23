use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

const SAVE_THRESHOLD: i128 = 100;

struct Map {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    cost: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    limit: usize,
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
            limit: 20,
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

    fn use_skill(&self, pos: (usize, usize)) -> usize {
        fn print_vst(vst: &Vec<Vec<usize>>) {
            for line in vst {
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
        let mut vst = vec![vec![usize::MAX; self.height]; self.width];
        let mut deque = VecDeque::from(vec![(pos.0 as i32, pos.1 as i32, 0)]);

        if self.map[pos.0][pos.1] == '#' {
            return 0;
        }

        let mut count = 0;
        while let Some((cx, cy, cc)) = deque.pop_front() {
            if cc >= vst[cx as usize][cy as usize] {
                continue;
            }
            vst[cx as usize][cy as usize] = cc;

            if self.map[cx as usize][cy as usize] != '#' {
                let old_cost = self.cost[pos.0][pos.1] as i128
                    - self.cost[cx as usize][cy as usize] as i128;
                let new_cost = cc as i128;
                let save = old_cost - new_cost;
                if save >= SAVE_THRESHOLD {
                    count += 1;
                    println!("({}, {}), {}", cx, cy, save);
                }
            }

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
                if nc >= vst[nx as usize][ny as usize] {
                    continue;
                }
                if nc > self.limit {
                    continue;
                }

                deque.push_back((nx, ny, nc));
            }
        }

        // print_vst(&vst);

        count
    }
}

fn main() {
    let mut map = Map::new();
    map.bfs_cost();
    map.print();

    let mut count = 0;
    for i in 0..map.width {
        for j in 0..map.height {
            let result = map.use_skill((i, j));
            // println!("save: {}", save);
            count += result;
        }
    }
    println!("count: {}", count);
}
