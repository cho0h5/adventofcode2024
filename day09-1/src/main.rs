use std::env;
use std::fs::read_to_string;

fn input() -> Vec<i64> {
    let mut disk = vec![];

    let filename = env::args().nth(1).unwrap();
    let binding = read_to_string(filename).unwrap();
    let line = binding.lines().next().unwrap();
    for (i, c) in line.chars().enumerate() {
        let num = c as i64 - '0' as i64;
        for _ in 0..num {
            if i % 2 == 0 {
                disk.push(i as i64 / 2);
            } else {
                disk.push(-1);
            }
        }
    }

    disk
}

fn compacting(disk: &mut [i64]) {
    let mut tail = disk.len() - 1;

    for i in 0..disk.len() {
        if disk[i] != -1 {
            continue;
        }

        while disk[tail] == -1 {
            tail -= 1;
        }

        if i >= tail {
            break;
        }

        disk[i] = disk[tail];
        disk[tail] = -1;
    }
}

fn checksum(disk: &[i64]) -> i64 {
    let mut sum = 0;

    for (i, id) in disk.iter().enumerate() {
        if *id == -1 {
            break;
        }

        sum += i as i64 * id;
    }

    sum
}

fn main() {
    let mut disk = input();
    println!("before: {:?}", disk);

    compacting(&mut disk);
    println!("after: {:?}", disk);

    let sum = checksum(&disk);
    println!("checksum: {:?}", sum);
}
