use std::env;
use std::fs::read_to_string;
use std::collections::VecDeque;

fn input() -> Vec<Vec<char>> {
    let mut map = vec![];

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        map.push(line.chars().collect());
    }

    map
}

fn print_map(map: &Vec<Vec<char>>) {
    println!("map:");
    for line in map {
        println!("{:?}", line);
    }
    println!();
}

fn bfs(map: &Vec<Vec<char>>, vst: &mut Vec<Vec<bool>>, x: usize, y: usize) -> (usize, usize) {
    let id = map[x][y];
    let count_perimeter = |x: usize, y: usize| -> usize {
        let mut perimeter = 0;
        map.get(x + 1).and_then(|row| row.get(y))
            .filter(|&&c| c == id)
            .map(|_| perimeter += 1);
        x.checked_sub(1).and_then(|nx| map.get(nx).and_then(|row| row.get(y)))
            .filter(|&&c| c == id)
            .map(|_| perimeter += 1);
        map.get(x).and_then(|row| row.get(y + 1))
            .filter(|&&c| c == id)
            .map(|_| perimeter += 1);
        y.checked_sub(1).and_then(|ny| map.get(x).and_then(|row| row.get(y - 1)))
            .filter(|&&c| c == id)
            .map(|_| perimeter += 1);
        4 - perimeter
    };
    let mut deque = VecDeque::from(vec![(x, y)]);
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((cur_x, cur_y)) = deque.pop_front() {
        if vst[cur_x][cur_y] {
            continue;
        }
        vst[cur_x][cur_y] = true;
        area += 1;
        perimeter += count_perimeter(cur_x, cur_y);

        let dl = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dl {
            let new_x = if dx < 0 {
                cur_x.checked_sub((-dx) as usize)
            } else {
                cur_x.checked_add(dx as usize)
            };
            let new_y = if dy < 0 {
                cur_y.checked_sub((-dy) as usize)
            } else {
                cur_y.checked_add(dy as usize)
            };

            if let (Some(nx), Some(ny)) = (new_x, new_y) {
                if nx < map.len()
                    && ny < map[0].len()
                    && !vst[nx][ny]
                    && map[nx][ny] == id {
                    deque.push_back((nx, ny));
                }
            }
        }
    }
    (area, perimeter)
}

fn main() {
    let mut map = input();
    print_map(&map);
    let mut vst: Vec<Vec<bool>> = map
        .iter()
        .map(|row| vec![false; row.len()])
        .collect();

    let mut total_price = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                continue;
            }

            let (area, perimeter) = bfs(&map, &mut vst, i, j);
            if area != 0 {
                println!("{}, {}: {}, {}", i, j, area, perimeter);
            }
            total_price += area * perimeter;
        }
    }
    println!("total_price: {}", total_price);
}
