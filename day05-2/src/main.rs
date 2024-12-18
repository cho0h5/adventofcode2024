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

fn generate_permutations(
    orders: &Vec<(i32, i32)>,
    update: &mut [i32],
    start: usize,
) -> i32 {
    if start == update.len() {
        // println!("start: {:?}", update);
        if solve_update(&orders, update) {
            return update[update.len() / 2];
        } else {
            return -1;
        }
    }

    for i in start..update.len() {
        update.swap(start, i);
        let res = generate_permutations(orders, update, start + 1);
        if res != -1 {
            return res;
        }
        update.swap(start, i);
    }
    -1
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

    let mut sum = 0;
    for update in incorrect_update.iter_mut() {
        let res = generate_permutations(&orders, &mut update[..], 0);
        println!("mid: {}", res);
        sum += res;
    }
    println!("sum: {}", sum);
}
