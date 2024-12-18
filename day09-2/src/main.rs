use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
enum FragState {
    File { size: usize, id: usize },
    Free { size: usize },
}

fn input() -> Vec<FragState> {
    let mut disk = Vec::new();

    let filename = env::args().nth(1).unwrap();
    let binding = read_to_string(filename).unwrap();
    let line = binding.lines().nth(0).unwrap();
    for (i, c) in line.chars().enumerate() {
        let num = c as usize - '0' as usize;
        if i % 2 == 0 {
            disk.push(FragState::File {
                size: num,
                id: i as usize / 2,
            });
        } else {
            disk.push(FragState::Free { size: num });
        }
    }

    disk
}

fn compacting(disk: &mut Vec<FragState>) {}

fn checksum(disk: &Vec<FragState>) -> usize {
    let mut sum = 0;

    let mut temp = Vec::new();
    for frag in disk {
        match frag {
            FragState::File { size, id } => {
                for _ in 0..*size {
                    temp.push(*id as i32);
                }
            }
            FragState::Free { size } => {
                for _ in 0..*size {
                    temp.push(-1);
                }
            }
        }
    }

    for (i, num) in temp.iter().enumerate() {
        if *num != -1 {
            sum += i * *num as usize;
        }
    }

    sum
}

fn print_disk(disk: &[FragState]) {
    let mut temp = Vec::new();
    for frag in disk {
        match frag {
            FragState::File { size, id } => {
                for _ in 0..*size {
                    temp.push(*id as i32);
                }
            }
            FragState::Free { size } => {
                for _ in 0..*size {
                    temp.push(-1);
                }
            }
        }
    }

    print!("disk: [");
    for num in temp {
        if num == -1 {
            print!("{} ", '.');
        } else {
            print!("{} ", num);
        }
    }
    println!("]");
}

fn main() {
    let mut disk = input();
    print_disk(&disk);

    compacting(&mut disk);
    print_disk(&disk);

    let sum = checksum(&disk);
    println!("checksum: {:?}", sum);
}
