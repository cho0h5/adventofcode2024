use std::env;
use std::fs::read_to_string;

fn input() -> (Vec<Vec<char>>, (i32, i32), Vec<char>) {
    let mut map = vec![];
    let mut start_point = (0, 0);
    let mut instructions: Vec<char> = vec![];

    let mut flag = false;
    let filename = env::args().nth(1).unwrap();
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        if !flag {
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
            if temp.is_empty() {
                flag = true;
            } else {
                map.push(temp);
            }
        } else {
            instructions.extend(line.chars());
        }
    }

    (map, start_point, instructions)
}

fn print_map(map: &[Vec<char>]) {
    for line in map {
        println!("{:?}", line);
    }
}

// `#`을 만나기 전까지 `.`이 있나 확인.
fn run(map: &mut [Vec<char>], pos: &mut (i32, i32), inst: char) {
    let width = map.len();
    let height = map[0].len();
    match inst {
        '>' => {
            let i = pos.0;
            let mut last_j = -1;
            for j in pos.1 + 1..height as i32 {
                if map[i as usize][j as usize] == '#' {
                    break;
                } else if map[i as usize][j as usize] == '.' {
                    last_j = j as i32;
                    break;
                }
            }
            if last_j < 0 {
                return;
            }
            for j in (pos.1..last_j).rev() {
                map[i as usize][j as usize + 1] = map[i as usize][j as usize]
            }
            pos.1 += 1;
        },
        '<' => {
            let i = pos.0;
            let mut last_j = -1;
            for j in (0..=pos.1 - 1).rev() {
                if map[i as usize][j as usize] == '#' {
                    break;
                } else if map[i as usize][j as usize] == '.' {
                    last_j = j as i32;
                    break;
                }
            }
            if last_j < 0 {
                return;
            }
            for j in last_j..pos.1 {
                map[i as usize][j as usize] = map[i as usize][j as usize + 1]
            }
            pos.1 -= 1;
        },
        'v' => {
            let j = pos.1;
            let mut last_i = -1;
            for i in pos.0 + 1..width as i32 {
                if map[i as usize][j as usize] == '#' {
                    break;
                } else if map[i as usize][j as usize] == '.' {
                    last_i = i as i32;
                    break;
                }
            }
            if last_i < 0 {
                return;
            }
            for i in (pos.0..last_i).rev() {
                map[i as usize + 1][j as usize] = map[i as usize][j as usize]
            }
            pos.0 += 1;
        },
        '^' => {
            let j = pos.1;
            let mut last_i = -1;
            for i in (0..=pos.0 - 1).rev() {
                if map[i as usize][j as usize] == '#' {
                    break;
                } else if map[i as usize][j as usize] == '.' {
                    last_i = i as i32;
                    break;
                }
            }
            if last_i < 0 {
                return;
            }
            for i in last_i..pos.0 {
                map[i as usize][j as usize] = map[i as usize + 1][j as usize]
            }
            pos.0 -= 1;
        },
        _ => (),
    }
}

fn sum_coordinates(map: &[Vec<char>]) -> usize {
    let mut coordinates = 0;
    for (i, line) in map.into_iter().enumerate() {
        for (j, c) in line.into_iter().enumerate() {
            if *c == 'O' {
                coordinates += i * 100 + j;
            }
        }
    }
    coordinates
}

fn main() {
    let (mut map, mut pos, instructions) = input();
    print_map(&map);
    println!("start_point: {:?}", pos);
    println!("instructions: {:?}", instructions);

    for inst in instructions {
        run(&mut map, &mut pos, inst);
        println!("----------------");
        println!("inst: {}", inst);
        println!("pos: {:?}", pos);
        print_map(&map);
    }
    let coordinates = sum_coordinates(&map);
    println!("----------------");
    println!("sum of coordinates: {}", coordinates);
}
