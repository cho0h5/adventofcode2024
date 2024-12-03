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
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
}

#[derive(PartialEq)]
enum Mul {
    Enabled,
    Disabled,
}

fn sol(line: &str, mut mul: Mul) -> (i32, Mul) {
    let mut state = State::S0;
    let mut param1 = String::new();
    let mut param2 = String::new();
    let mut sum = 0;

    for c in line.chars() {
        match state {
            _ if c == 'd' => state = State::S8,
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
                if mul == Mul::Enabled {
                    sum += num1 * num2;
                }
                state = State::S0;
            },
            State::S8 if c == 'o' => state = State::S9,
            State::S9 if c == 'n' => state = State::S10,
            State::S9 if c == '(' => state = State::S14,
            State::S10 if c == '\'' => state = State::S11,
            State::S11 if c == 't' => state = State::S12,
            State::S12 if c == '(' => state = State::S13,
            State::S13 if c == ')' => {
                mul = Mul::Disabled;
                println!("Disabled");
                state = State::S0;
            },
            State::S14 if c == ')' => {
                mul = Mul::Enabled;
                println!("Enabled");
                state = State::S0;
            },
            _ => {
                param1.clear();
                param2.clear();
                state = State::S0;
            },
        }
    }

    (sum, mul)
}

fn main() {
    let filename = "input";

    let mut sum = 0;
    let mut mul = Mul::Enabled;
    for line in read_to_string(filename).unwrap().lines() {
        let ret = sol(line, mul);
        let sub = ret.0;
        mul = ret.1;
        println!("sub: {}", sub);
        sum += sub;
    }

    println!("{}", sum);
}
