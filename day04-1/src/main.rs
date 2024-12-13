use std::env;
use std::fs::read_to_string;

fn subsub(shirt: &Vec<Vec<char>>, target: &str,
    di: isize, dj: isize,
    mut i: isize, mut j: isize) -> bool {
    let h = shirt.len() as isize;
    let w = shirt[0].len() as isize;

    for c in target.chars() {
        if i < 0 || j < 0 || i >= h || j >= w {
            return false;
        }

        if shirt[i as usize][j as usize] != c {
            return false;
        }

        i += di;
        j += dj;
    }
    true
}

fn subtask(shirt: &Vec<Vec<char>>, target: &str, i: isize, j: isize) -> usize{
    let mut count = 0;

    if subsub(shirt, target, -1, 0, i, j) {
        count += 1;
    }
    if subsub(shirt, target, -1, 1, i, j) {
        count += 1;
    }
    if subsub(shirt, target, 0, 1, i, j) {
        count += 1;
    }
    if subsub(shirt, target, 1, 1, i, j) {
        count += 1;
    }
    if subsub(shirt, target, 1, 0, i, j) {
        count += 1;
    }
    if subsub(shirt, target, 1, -1, i, j) {
        count += 1;
    }
    if subsub(shirt, target, 0, -1, i, j) {
        count += 1;
    }
    if subsub(shirt, target, -1, -1, i, j) {
        count += 1;
    }

    count
}

fn sol(shirt: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for (i, line) in shirt.iter().enumerate() {
        for j in 0..line.len() {
            count += subtask(shirt, "XMAS", i as isize, j as isize);
        }
    }

    count
}

fn main() {
    let mut shirt = vec![];

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        shirt.push(line.chars().collect::<Vec<_>>());
    }

    let answer = sol(&shirt);
    println!("{}", answer);
}
