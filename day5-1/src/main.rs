use std::env;
use std::fs::read_to_string;

fn main() {
    let mut orders = vec![];
    let mut updates = vec![];

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<&str> = line.split('|').collect();

        if numbers.len() != 1 {
            let num1 = numbers[0].parse::<i32>().unwrap();
            let num2 = numbers[1].parse::<i32>().unwrap();
            orders.push((num1, num2));
        } else {
            let numbers: Vec<&str> = line.split(',').collect();
            if numbers.len() == 1 {
                continue;
            }
            let numbers: Vec<i32> = numbers.iter()
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect();
            updates.push(numbers);
        }
    }

    println!("{:?}", orders);
    println!("{:?}", updates);
}
