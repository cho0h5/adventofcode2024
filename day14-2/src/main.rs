use std::env;
use std::fs::read_to_string;

// const WIDTH: usize = 11;
// const HEIGHT: usize = 7;
const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const DURATION: usize = 1000;

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

fn print_map(robots: &[Robot]) {
    let mut map = vec![vec![0; HEIGHT]; WIDTH];

    for robot in robots {
        let px = robot.pos_x as usize;
        let py = robot.pos_y as usize;
        map[px][py] += 1;
    }

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            if map[i][j] != 0 {
                print!("{} ", map[i][j]);
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() {
    let mut robots = parse_file();

    for i in 0..DURATION {
        println!("i: {}", i);
        simulation(&mut robots);
        print_map(&robots);
        println!("----------------------------------------");
    }
}
