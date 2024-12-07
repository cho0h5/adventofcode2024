use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone)]
struct Guard {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Guard {
    fn get_dx_dy(&self) -> (isize, isize) {
        match self.dir {
            Direction::N => (-1, 0),
            Direction::E => (0, 1),
            Direction::S => (1, 0),
            Direction::W => (0, -1),
        }
    }

    fn turn(&mut self) {
        match self.dir {
            Direction::N => self.dir = Direction::E,
            Direction::E => self.dir = Direction::S,
            Direction::S => self.dir = Direction::W,
            Direction::W => self.dir = Direction::N,
        }
    }

    fn is_valid_coor(pos: (isize, isize), map_size: (isize, isize)) -> bool {
        pos.0 < 0 || pos.0 >= map_size.0 || pos.1 < 0 || pos.1 >= map_size.1
    }

    fn forward(&mut self, map: &Vec<Vec<char>>) -> bool {
        let d = self.get_dx_dy();
        let map_size = (map.len() as isize, map[0].len() as isize);
        let next_pos = (self.x + d.0, self.y + d.1);
        if Self::is_valid_coor(next_pos, map_size) {
            return false;
        }

        let next_cell = map[next_pos.0 as usize][next_pos.1 as usize];
        if next_cell == '#' {
            self.turn();
        } else {
            self.x = next_pos.0;
            self.y = next_pos.1;
        }
        true
    }
}

fn is_stuck(map: &Vec<Vec<char>>, mut guard: Guard) -> bool {
    for _ in 0..20000 {
        if !guard.forward(map) {
            return false;
        }
    }
    return true;
}

fn input() -> (Vec<Vec<char>>, Guard) {
    let mut map = vec![];
    let mut x = 0;
    let mut y = 0;

    let filename = env::args().nth(1).unwrap();
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        map.push(line.chars().collect::<Vec<_>>());

        for (j, c) in map[i].iter_mut().enumerate() {
            if *c == '^' {
                x = i;
                y = j;
                *c = '.';
            }
        }
    }

    (
        map,
        Guard {
            x: x as isize,
            y: y as isize,
            dir: Direction::N,
        },
    )
}

fn sol(map: &mut Vec<Vec<char>>, guard: &Guard) -> i32 {
    let h = map.len();
    let w = map[0].len();

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '.' {
                map[i][j] = '#';
                if is_stuck(&map, guard.clone()) {
                    count += 1;
                }
                map[i][j] = '.';
            }
        }
    }
    count
}

fn main() {
    let (mut map, guard) = input();
    let count = sol(&mut map, &guard);
    println!("answer: {}", count);
}
