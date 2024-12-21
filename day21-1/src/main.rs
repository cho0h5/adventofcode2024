use std::env;
use std::fs::read_to_string;

fn input() -> Vec<String> {
    let mut codes = Vec::new();

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        codes.push(line.to_owned());
    }

    codes
}

fn resolve_numeric_keypad(code: &str) -> Vec<String> {
    vec![String::from("foo")]
}

fn resolve_directional_keypad(code: &str) -> String {
    String::from("bar")
}

fn main() {
    let codes = input();

    for code in &codes {
        let resolved1s = resolve_numeric_keypad(code);
        println!("----------------");
        println!("code: {}", code);
        for resolved1 in resolved1s {
            let resolved2 = resolve_directional_keypad(&resolved1);
            let resolved3 = resolve_directional_keypad(&resolved2);
            println!("resolved1: {}", resolved1);
            println!("resolved2: {}", resolved2);
            println!("resolved3: {}", resolved3);
        }
    }
}
