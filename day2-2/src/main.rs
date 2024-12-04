use std::fs::read_to_string;

fn is_increasing(numbers: &Vec<i32>) -> bool {
    for i in 1..numbers.len() {
        let gap = numbers[i] - numbers[i - 1];
        if gap <= 0 || gap >= 4{
            return false;
        }
    }
    return true;
}

fn is_decreasing(numbers: &Vec<i32>) -> bool {
    for i in 1..numbers.len() {
        let gap = numbers[i] - numbers[i - 1];
        if gap >= 0 || gap <= -4 {
            return false;
        }
    }
    return true;
}

fn bf(numbers: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut numberss = vec![];

    for i in 0..numbers.len() {
        let mut temp = numbers[0..i].to_vec();
        temp.extend(&numbers[i + 1..]);
        numberss.push(temp);
    }

    numberss
}

fn main() {
    let filename = "input";

    let mut count = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();

        let numberss = bf(&numbers);

        for numbers in numberss {
            if is_increasing(&numbers)
                || is_decreasing(&numbers) {
                count += 1;
                println!("Safe");
                break;
            } else {
                println!("Unsafe");
            }
        }
    }
    println!("{}", count);
}
