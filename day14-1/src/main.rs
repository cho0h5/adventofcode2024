use std::env;
use std::fs::read_to_string;

// const WIDTH: usize = 11;
// const HEIGHT: usize = 7;
const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const DURATION: usize = 100;

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

// 0 1 2 3 4 5 6 7 8 9 A B
// 1
// 2
// 3
// 4
// 5
// 6
// 7

fn count_in_each_quadrant(robots: &[Robot]) -> (u32, u32, u32, u32) {
    let width_mid = WIDTH as i32 / 2;
    let height_mid = HEIGHT as i32 / 2;

    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;
    let mut c4 = 0;

    for robot in robots {
        match (robot.pos_x, robot.pos_y) {
            (x, y) if x < width_mid && y < height_mid => c1 += 1,
            (x, y) if x > width_mid && y < height_mid => c2 += 1,
            (x, y) if x < width_mid && y > height_mid => c3 += 1,
            (x, y) if x > width_mid && y > height_mid => c4 += 1,
            _ => (),
        }
    }
    (c1, c2, c3, c4)
}

fn main() {
    let mut robots = parse_file();
    // println!("{:?}", robots);

    for _ in 0..DURATION {
        simulation(&mut robots);
    }
    // println!("{:?}", robots);

    let count = count_in_each_quadrant(&robots);
    println!("{:?}", count);
    println!("safety factor: {}", count.0 * count.1 * count.2 * count.3);
}
