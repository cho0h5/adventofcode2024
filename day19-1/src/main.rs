use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

fn input() -> (Vec<String>, Vec<String>) {
    let mut wanteds = Vec::new();

    let filename = env::args().nth(1).unwrap();
    let binding = read_to_string(filename).unwrap();
    let mut iterator = binding.lines();
    let mut availables = iterator
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_owned())
        .collect();
    iterator.next();
    for line in iterator {
        wanteds.push(line.to_owned());
    }
    (availables, wanteds)
}

fn is_possible(
    availables: &[String],
    wanted: &str,
    pos: usize,
    count: &mut usize,
) -> bool {
    if wanted.len() == pos {
        return true;
    }
    if *count > 10337031 {
        return false;
    }

    for avail in availables {
        if wanted[pos..].starts_with(avail) {
            // println!("looip: {}, {}, {}", pos, &wanted[..pos], count);
            if is_possible(availables, wanted, pos + avail.len(), count) {
                return true;
            }
        }
        *count += 1;
    }
    false
}

fn main() {
    let (availables, wanteds) = input();
    println!("{:?}", availables);
    // println!("{:?}", wanteds);

    let mut count = 0;
    for wanted in wanteds {
        let is_possible = is_possible(&availables, &wanted, 0, &mut 0);
        println!("{} <- {}", is_possible, wanted);
        if is_possible {
            count += 1;
        }
    }
    println!("count: {}", count);
}
