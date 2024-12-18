use std::env;
use std::fs::read_to_string;

fn solve_update(orders: &Vec<(i32, i32)>, update: &[i32]) -> bool {
    fn is_valid(orders: &Vec<(i32, i32)>, a: i32, b: i32) -> bool {
        for order in orders {
            if order.0 == b && order.1 == a {
                return false;
            }
        }
        true
    }

    let len = update.len();

    for i in 0..len {
        for j in (i + 1)..len {
            if !is_valid(orders, update[i], update[j]) {
                return false;
            }
        }
    }

    true
}

fn input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
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
            let numbers: Vec<i32> = numbers
                .iter()
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect();
            updates.push(numbers);
        }
    }
    (orders, updates)
}

fn sort_update(orders: &Vec<(i32, i32)>, update: &mut Vec<i32>) -> Vec<i32> {
    fn find_minimum(orders: &Vec<(i32, i32)>, update: &Vec<i32>) -> usize {
        if update.len() == 1 {
            return 0;
        }
        'outer: for (i, num1) in update.iter().enumerate() {
            for (j, num2) in update.iter().enumerate() {
                if i == j {
                    continue;
                }
                for order in orders {
                    if order.0 == *num2 && order.1 == *num1 {
                        // println!("critical: {:?}", order);
                        continue 'outer;
                    } else if order.0 == *num1 && order.1 == *num2 {
                        // println!("pass: {:?}", order);
                    }
                }
            }
            return i;
        }
        usize::MAX
    };

    let mut sorted = Vec::new();
    while !update.is_empty() {
        // println!("curr update: {:?}", update);
        let minimum = find_minimum(orders, update);
        sorted.push(update[minimum]);
        update.remove(minimum);
    }
    sorted
}

fn main() {
    let (orders, updates) = input();
    let mut incorrect_update = vec![];
    for update in updates.iter() {
        if solve_update(&orders, update) {
            println!("passed");
        } else {
            println!("failed: {:?}", update);
            incorrect_update.push(update.clone());
        }
    }
    println!("----------------");

    let mut sum = 0;
    for update in incorrect_update.iter_mut() {
        let sorted_update = sort_update(&orders, update);
        println!("sorted update: {:?}", sorted_update);
        let mid = sorted_update[sorted_update.len() / 2];
        println!("mid: {}", mid);
        sum += mid;
    }
    println!("sum: {}", sum);
}
