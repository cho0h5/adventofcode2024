use std::env;
use std::fs::read_to_string;

fn subsub(shirt: &Vec<Vec<char>>, target: &str, i: usize, j: usize) -> bool {
    shirt[i - 1][j + 1] == target.chars().nth(0).unwrap()
        && shirt[i + 1][j + 1] == target.chars().nth(1).unwrap()
        && shirt[i + 1][j - 1] == target.chars().nth(2).unwrap()
        && shirt[i - 1][j - 1] == target.chars().nth(3).unwrap()
}

fn subtask(shirt: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if shirt[i][j] != 'A' {
        return false;
    }

    let h = shirt.len();
    let w = shirt[0].len();
    if i <= 0 || i >= h - 1 || j <= 0 || j >= w - 1 {
        return false;
    }

    subsub(shirt, "MSSM", i, j)
        || subsub(shirt, "SSMM", i, j)
        || subsub(shirt, "SMMS", i, j)
        || subsub(shirt, "MMSS", i, j)
}

fn sol(shirt: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for (i, line) in shirt.iter().enumerate() {
        for j in 0..line.len() {
            if subtask(shirt, i, j) {
                count += 1;
            }
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

