use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

fn input() -> (Vec<Vec<char>>, (i32, i32), Vec<char>) {
    let mut map = vec![];
    let mut start_point = (0, 0);

    let filename = env::args().nth(1).unwrap();
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        let mut temp = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    start_point = (i as i32, j as i32);
                    temp.push('.');
                }
                c => {
                    temp.push(c);
                }
            }
        }
        if !temp.is_empty() {
            map.push(temp);
        }
    }

    let instructions = map.pop().unwrap();

    (map, start_point, instructions)
}

fn print_map(map: &[Vec<char>]) {
    for line in map {
        println!("{:?}", line);
    }
}

fn main() {
    let (mut map, start_point, instructions) = input();
    print_map(&map);
    println!("start_point: {:?}", start_point);
    println!("instructions: {:?}", instructions);
}
