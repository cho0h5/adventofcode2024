use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

const WIDTH: usize = 7;
const HEIGHT: usize = 7;

struct Computer {
    memory: Vec<Vec<char>>,
    coords: VecDeque<(usize, usize)>,
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

    let mut memory = vec![vec!['.'; HEIGHT]; WIDTH];
    Computer { memory, coords }
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

    fn step(&mut self) {
        if let Some((x, y)) = self.coords.pop_front() {
            self.memory[x][y] = '#';
        }
    }
}

fn main() {
    let mut computer = input();
    println!("coords: {:?}", computer.coords);
    computer.print_memory();

    for i in 0..computer.coords.len() + 1 {
        println!("------ i: {} ------", i);
        computer.step();
        computer.print_memory();
    }
}
