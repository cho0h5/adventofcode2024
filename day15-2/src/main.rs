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
                        start_point = (i as i32, j as i32 * 2);
                        temp.push('.');
                        temp.push('.');
                    }
                    'O' => {
                        temp.push('[');
                        temp.push(']');
                    }
                    c => {
                        temp.push(c);
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

fn print_map(map: &[Vec<char>], pos: (i32, i32)) {
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if i == pos.0 as usize && j == pos.1 as usize {
                print!("@");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn move_right(map: &mut [Vec<char>], x: i32, y: i32) -> bool {
    let current_char = map[x as usize][y as usize];
    match current_char {
        '.' => true,
        '[' if move_right(map, x, y + 2) => {
            map[x as usize][y as usize + 0] = '.';
            map[x as usize][y as usize + 1] = '[';
            map[x as usize][y as usize + 2] = ']';
            true
        }
        _ => false,
    }
}

fn move_left(map: &mut [Vec<char>], x: i32, y: i32) -> bool {
    let current_char = map[x as usize][y as usize];
    match current_char {
        '.' => true,
        ']' if move_left(map, x, y - 2) => {
            map[x as usize][y as usize + 0] = '.';
            map[x as usize][y as usize - 1] = ']';
            map[x as usize][y as usize - 2] = '[';
            true
        }
        _ => false,
    }
}

fn move_up_check(map: &[Vec<char>], x: i32, y: i32) -> bool {
    let current_char = map[x as usize][y as usize];
    match current_char {
        '.' => true,
        '[' if move_up_check(map, x - 1, y)
            && move_up_check(map, x - 1, y + 1) =>
        {
            true
        }
        ']' if move_up_check(map, x - 1, y - 1)
            && move_up_check(map, x - 1, y) =>
        {
            true
        }
        _ => false,
    }
}

fn move_up(map: &mut [Vec<char>], x: i32, y: i32) {
    let current_char = map[x as usize][y as usize];
    match current_char {
        '[' => {
            move_up(map, x - 1, y);
            move_up(map, x - 1, y + 1);
            map[x as usize][y as usize] = '.';
            map[x as usize][y as usize + 1] = '.';
            map[x as usize - 1][y as usize] = '[';
            map[x as usize - 1][y as usize + 1] = ']';
        }
        ']' => {
            move_up(map, x - 1, y - 1);
            move_up(map, x - 1, y);
            map[x as usize][y as usize] = '.';
            map[x as usize][y as usize - 1] = '.';
            map[x as usize - 1][y as usize] = ']';
            map[x as usize - 1][y as usize - 1] = '[';
        }
        _ => (),
    }
}

fn move_down_check(map: &[Vec<char>], x: i32, y: i32) -> bool {
    let current_char = map[x as usize][y as usize];
    match current_char {
        '.' => true,
        '[' if move_down_check(map, x + 1, y)
            && move_down_check(map, x + 1, y + 1) =>
        {
            true
        }
        ']' if move_down_check(map, x + 1, y - 1)
            && move_down_check(map, x + 1, y) =>
        {
            true
        }
        _ => false,
    }
}

fn move_down(map: &mut [Vec<char>], x: i32, y: i32) {
    let current_char = map[x as usize][y as usize];
    match current_char {
        '[' => {
            move_down(map, x + 1, y);
            move_down(map, x + 1, y + 1);
            map[x as usize][y as usize] = '.';
            map[x as usize][y as usize + 1] = '.';
            map[x as usize + 1][y as usize] = '[';
            map[x as usize + 1][y as usize + 1] = ']';
        }
        ']' => {
            move_down(map, x + 1, y - 1);
            move_down(map, x + 1, y);
            map[x as usize][y as usize] = '.';
            map[x as usize][y as usize - 1] = '.';
            map[x as usize + 1][y as usize] = ']';
            map[x as usize + 1][y as usize - 1] = '[';
        }
        _ => (),
    }
}

fn run(map: &mut [Vec<char>], pos: &mut (i32, i32), inst: char) {
    match inst {
        '>' if move_right(map, pos.0, pos.1 + 1) => pos.1 += 1,
        '<' if move_left(map, pos.0, pos.1 - 1) => pos.1 -= 1,
        'v' if move_down_check(map, pos.0 + 1, pos.1) => {
            move_down(map, pos.0 + 1, pos.1);
            pos.0 += 1;
        }
        '^' if move_up_check(map, pos.0 - 1, pos.1) => {
            move_up(map, pos.0 - 1, pos.1);
            pos.0 -= 1;
        }
        _ => (),
    }
}

fn sum_coordinates(map: &[Vec<char>]) -> usize {
    let mut coordinates = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '[' {
                coordinates += i * 100 + j;
            }
        }
    }
    coordinates
}

fn main() {
    let (mut map, mut pos, instructions) = input();
    print_map(&map, pos);
    println!("start_point: {:?}", pos);
    println!("instructions: {:?}", instructions);

    for inst in instructions {
        run(&mut map, &mut pos, inst);
        println!("----------------");
        println!("inst: {}", inst);
        println!("pos: {:?}", pos);
        print_map(&map, pos);
    }
    let coordinates = sum_coordinates(&map);
    println!("----------------");
    println!("sum of coordinates: {}", coordinates);
}
