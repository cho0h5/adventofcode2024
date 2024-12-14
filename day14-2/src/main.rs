use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

// const WIDTH: usize = 11;
// const HEIGHT: usize = 7;
const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

fn parse_file() -> Vec<Robot> {
    let filename = env::args().nth(1).unwrap();
    let mut robots = Vec::new();
    let mut robot = Robot {
        pos_x: 0,
        pos_y: 0,
        vel_x: 0,
        vel_y: 0,
    };

    for line in read_to_string(filename).unwrap().lines() {
        if let Some((pos, vel)) =
            line.strip_prefix("p=").unwrap().split_once(" v=")
        {
            if let Some((px, py)) = pos.split_once(",") {
                robot.pos_x = px.parse().unwrap();
                robot.pos_y = py.parse().unwrap();
            }

            if let Some((vx, vy)) = vel.split_once(",") {
                robot.vel_x = vx.parse().unwrap();
                robot.vel_y = vy.parse().unwrap();
            }

            robots.push(robot);
        }
    }

    robots
}

fn simulation(robots: &mut [Robot]) {
    for robot in robots {
        robot.pos_x += robot.vel_x + WIDTH as i32;
        robot.pos_x %= WIDTH as i32;

        robot.pos_y += robot.vel_y + HEIGHT as i32;
        robot.pos_y %= HEIGHT as i32;
    }
}

fn create_map(robots: &[Robot]) -> Vec<Vec<usize>> {
    let mut map = vec![vec![0; HEIGHT]; WIDTH];

    for robot in robots {
        let px = robot.pos_x as usize;
        let py = robot.pos_y as usize;
        map[px][py] += 1;
    }
    map
}

fn print_map(map: &[Vec<usize>]) {
    for j in 0..HEIGHT {
        for line in map {
            if line[j] != 0 {
                print!("{} ", line[j]);
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn bfs(
    map: &[Vec<usize>],
    vst: &mut [Vec<bool>],
    sx: usize,
    sy: usize,
) -> usize {
    let mut deque = VecDeque::new();
    deque.push_back((sx, sy));

    let mut area = 0;
    while let Some((cx, cy)) = deque.pop_front() {
        let dx = [1, 1, 1, 0, -1, -1, -1, 0];
        let dy = [-1, 0, 1, 1, 1, 0, -1, -1];

        for k in 0..8 {
            let nx = cx as i32 + dx[k];
            let ny = cy as i32 + dy[k];

            if nx < 0 || nx >= WIDTH as i32 || ny < 0 || ny >= HEIGHT as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if map[nx][ny] == 0 || vst[nx][ny] {
                continue;
            }

            vst[nx][ny] = true;
            area += 1;
            deque.push_back((nx, ny));
        }
    }
    area
}

fn is_easter_egg(map: &[Vec<usize>]) -> usize {
    let mut vst = vec![vec![false; HEIGHT]; WIDTH];

    let mut maximum = 0;
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if map[i][j] == 0 || vst[i][j] {
                continue;
            }

            let res = bfs(map, &mut vst, i, j);
            maximum = std::cmp::max(maximum, res);
        }
    }
    maximum
}

const DURATION: usize = 1000000;

fn main() {
    let mut robots = parse_file();

    for i in 0..DURATION {
        simulation(&mut robots);
        let map = create_map(&robots);
        let easter_egg_score = is_easter_egg(&map);
        if easter_egg_score >= 100 {
            print_map(&map);
            println!("easter egg score: {}", easter_egg_score);
            println!("elapsed second: {}", i + 1);
            println!("----------------------------------------");
            break;
        }
    }
}
