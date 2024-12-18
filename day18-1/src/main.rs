use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

const WIDTH: usize = 7;
const HEIGHT: usize = 7;

struct Computer {
    memory: Vec<Vec<char>>,
    vst: Vec<Vec<usize>>,
    coords: VecDeque<(usize, usize)>,
    deque: VecDeque<(i32, i32, usize)>,
}

fn input() -> Computer {
    let mut coords = VecDeque::new();

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        let (part_x, part_y) = line.split_once(',').unwrap();
        let x = part_x.parse::<usize>().unwrap();
        let y = part_y.parse::<usize>().unwrap();
        coords.push_back((x, y));
    }

    let memory = vec![vec!['.'; HEIGHT]; WIDTH];
    let vst = vec![vec![usize::MAX; HEIGHT]; WIDTH];
    let deque = VecDeque::from(vec![(0, 0, 1)]);

    Computer {
        memory,
        vst,
        coords,
        deque,
    }
}

impl Computer {
    fn print_memory(&self) {
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                print!("{}", self.memory[i][j]);
            }
            println!();
        }
    }

    fn collapse_step(&mut self) {
        if let Some((x, y)) = self.coords.pop_front() {
            self.memory[x][y] = '#';
        }
    }

    fn bfs_step(&mut self) -> Option<usize> {
        let mut next_deque = VecDeque::new();

        while let Some((cx, cy, cc)) = self.deque.pop_front() {
            if self.vst[cx as usize][cy as usize] <= cc {
                continue;
            }
            self.vst[cx as usize][cy as usize] = cc;
            self.memory[cx as usize][cy as usize] = 'O';

            let dx = [0, 1, 0, -1];
            let dy = [1, 0, -1, 0];
            for i in 0..4 {
                let nx = cx + dx[i];
                let ny = cy + dy[i];
                let nc = cc + 1;

                if nx < 0 || nx >= WIDTH as i32 || ny < 0 || ny >= HEIGHT as i32
                {
                    continue;
                }
                if self.memory[nx as usize][ny as usize] == '#' {
                    continue;
                }
                if self.vst[nx as usize][ny as usize] <= nc {
                    continue;
                }
                next_deque.push_back((nx, ny, nc));
            }
        }
        self.deque = next_deque;

        if self.vst[WIDTH - 1][HEIGHT - 1] != usize::MAX {
            Some(self.vst[WIDTH - 1][HEIGHT - 1])
        } else {
            None
        }
    }
}

fn main() {
    let mut computer = input();
    println!("coords: {:?}", computer.coords);
    computer.print_memory();

    for i in 0..computer.coords.len() + 1 {
        println!("------ i: {} ------", i);
        if let Some(cost) = computer.bfs_step() {
            println!("cost: {}", cost);
            break;
        }
        computer.collapse_step();
        computer.print_memory();
    }
}
