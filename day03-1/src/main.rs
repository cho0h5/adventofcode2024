use std::fs::read_to_string;

enum State {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
}

fn sol(line: &str) -> i32 {
    let mut state = State::S0;
    let mut param1 = String::new();
    let mut param2 = String::new();
    let mut sum = 0;

    for c in line.chars() {
        match state {
            State::S0 if c == 'm' => state = State::S1,
            State::S1 if c == 'u' => state = State::S2,
            State::S2 if c == 'l' => state = State::S3,
            State::S3 if c == '(' => state = State::S4,
            State::S4 if c.is_ascii_digit() => {
                param1.push(c);
                state = State::S5;
            },
            State::S5 if c.is_ascii_digit() => param1.push(c),
            State::S5 if c == ',' => state = State::S6,
            State::S6 if c.is_ascii_digit() => {
                param2.push(c);
                state = State::S7;
            },
            State::S7 if c.is_ascii_digit() => param2.push(c),
            State::S7 if c == ')' => {
                let num1: i32 = param1.parse().unwrap();
                let num2: i32 = param2.parse().unwrap();
                param1.clear();
                param2.clear();
                sum += num1 * num2;
                state = State::S0;
            },
            _ => {
                param1.clear();
                param2.clear();
                state = State::S0;
            },
        }
    }

    sum
}

fn main() {
    let filename = "input";

    let mut sum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let sub = sol(line);
        println!("sub: {}", sub);
        sum += sub;
    }

    println!("{}", sum);
}
