use std::env;
use std::fs::read_to_string;

fn input() -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let filename = env::args().nth(1).unwrap();
    for lock_or_key in read_to_string(filename).unwrap().split("\n\n") {
        let mut iter = lock_or_key.lines();

        if iter.next().unwrap().starts_with('#') {
            // lock
            let schemetic: Vec<Vec<_>> =
                iter.map(|line| line.chars().collect()).collect();

            let mut heights = [5; 5];
            for (j, height) in heights.iter_mut().enumerate() {
                for (i, line) in schemetic.iter().enumerate() {
                    if *height == 5 && line[j] == '.' {
                        *height = i;
                    }
                }
            }

            println!("lock: {:?}", heights);
            locks.push(heights);
        } else {
            // key
            let schemetic: Vec<Vec<_>> =
                iter.map(|line| line.chars().collect()).collect();

            let mut heights = [5; 5];
            for (j, height) in heights.iter_mut().enumerate() {
                for (i, line) in schemetic.iter().enumerate().rev() {
                    if *height == 5 && line[j] == '.' {
                        *height = 4 - i;
                    }
                }
            }

            println!("key: {:?}", heights);
            keys.push(heights);
        }
    }

    (locks, keys)
}

fn is_matched(lock: &[usize; 5], key: &[usize; 5]) -> bool {
    for (a, b) in lock.iter().zip(key.iter()) {
        if a + b > 5 {
            return false;
        }
    }
    true
}

fn main() {
    let (locks, keys) = input();
    println!("locks len: {:?}", locks.len());
    println!("keys len: {:?}", keys.len());

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if is_matched(lock, key) {
                count += 1;
            }
        }
    }
    println!("count: {}", count);
}
