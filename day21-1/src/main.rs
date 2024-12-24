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

struct NumKey;

impl NumKey {
    fn find_numkey_pos(ch: char) -> Option<(i32, i32)> {
        let numkey = [
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec![' ', '0', 'A'],
        ];

        for (i, line) in numkey.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if ch == *c {
                    return Some((i as i32, j as i32));
                }
            }
        }
        None
    }

    fn gen_numkey_path(from: char, to: char) -> Vec<String> {
        let from_pos = Self::find_numkey_pos(from).unwrap();
        let to_pos = Self::find_numkey_pos(to).unwrap();

        let dx = to_pos.0 - from_pos.0;
        let dy = to_pos.1 - from_pos.1;

        let vertical = if dx < 0 {
            "^".repeat(-dx as usize)
        } else {
            "v".repeat(dx as usize)
        };

        let horizontal = if dy < 0 {
            "<".repeat(-dy as usize)
        } else {
            ">".repeat(dy as usize)
        };

        if vertical.is_empty() {
            vec![horizontal]
        } else if horizontal.is_empty() {
            vec![vertical]
        } else if from_pos.0 == 3 && to_pos.1 == 0 {
            vec![vertical + &horizontal]
        } else if from_pos.1 == 0 && to_pos.0 == 3 {
            vec![horizontal + &vertical]
        } else {
            vec![vertical.clone() + &horizontal, horizontal + &vertical]
        }
    }

    fn cal_numkey(code: &str) {
        let code = String::from("A") + code;
        let code = code.as_bytes();

        for i in 0..code.len() - 1 {
            println!("{} {}", code[i] as char, code[i + 1] as char);
            let paths =
                Self::gen_numkey_path(code[i] as char, code[i + 1] as char);
            println!("{:?}", paths);
        }
    }
}

fn main() {
    let codes = input();

    println!("codes: {:?}", codes);
    NumKey::cal_numkey(&codes[0]);
}
