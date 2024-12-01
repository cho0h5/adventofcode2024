use std::fs::read_to_string;

fn main() {
    let filename = "input";
    let mut vec1 = vec![];
    let mut vec2 = vec![];

    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        vec1.push(numbers[0].parse::<i32>().unwrap());
        vec2.push(numbers[1].parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();

    let mut distance = 0;
    for (a, b) in vec1.iter().zip(vec2.iter()) {
        distance += (a - b).abs();
    }
    println!("{}", distance);
}
