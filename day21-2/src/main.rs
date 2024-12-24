use std::collections::HashMap;
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
    fn find_key_pos(ch: char) -> Option<(i32, i32)> {
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
        let from_pos = Self::find_key_pos(from).unwrap();
        let to_pos = Self::find_key_pos(to).unwrap();

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

    fn cal_key(dirkey: &mut DirKey, code: &str) -> usize {
        let code = code.as_bytes();

        let mut cost = 0;
        for i in 0..code.len() - 1 {
            let paths =
                Self::gen_numkey_path(code[i] as char, code[i + 1] as char);
            // println!("--------");
            // println!(
            // "from: {}, to: {}, paths: {:?}",
            // code[i] as char,
            // code[i + 1] as char,
            // paths
            // );

            let mut minimum = usize::MAX;
            for path in &paths {
                let path = String::from("A") + path + "A";
                // println!("current path: {:?}", path);
                let c = dirkey.cal_key(&path, 25);
                minimum = std::cmp::min(minimum, c);
            }
            // println!("minimum cost: {}", minimum);
            cost += minimum;
        }
        cost
    }
}

struct DirKey(HashMap<(char, char, usize), usize>);

impl DirKey {
    fn new() -> Self {
        DirKey(HashMap::new())
    }

    fn find_key_pos(ch: char) -> Option<(i32, i32)> {
        let dirkey = [vec![' ', '^', 'A'], vec!['<', 'v', '>']];

        for (i, line) in dirkey.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if ch == *c {
                    return Some((i as i32, j as i32));
                }
            }
        }
        None
    }

    fn gen_key_path(from: char, to: char) -> Vec<String> {
        let from_pos = Self::find_key_pos(from).unwrap();
        let to_pos = Self::find_key_pos(to).unwrap();

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
        } else if from_pos.0 == 0 && to_pos.1 == 0 {
            vec![vertical + &horizontal]
        } else if from_pos.1 == 0 && to_pos.0 == 0 {
            vec![horizontal + &vertical]
        } else {
            vec![vertical.clone() + &horizontal, horizontal + &vertical]
        }
    }

    fn cal_key(&mut self, code: &str, step: usize) -> usize {
        if step == 0 {
            return code.len() - 1;
        }

        let code = code.as_bytes();

        let mut total_cost = 0;
        for i in 0..code.len() - 1 {
            let from = code[i] as char;
            let to = code[i + 1] as char;

            if let Some(cost) = self.0.get(&(from, to, step)) {
                total_cost += cost;
            } else {
                let paths = Self::gen_key_path(from, to);

                let mut minimum = usize::MAX;
                for path in &paths {
                    let path = String::from("A") + path + "A";
                    let c = self.cal_key(&path, step - 1);
                    minimum = std::cmp::min(minimum, c);
                }
                self.0.insert((from, to, step), minimum);
                total_cost += minimum;
            }
        }
        total_cost
    }
}

fn main() {
    let codes = input();
    let mut dirkey = DirKey::new();

    let mut total_complexity = 0;
    for code in &codes {
        println!("----------------");
        let numeric_part: usize = code[..code.len() - 1].parse().unwrap();
        let code = String::from("A") + code;
        let cost = NumKey::cal_key(&mut dirkey, &code);
        println!("cost: {}, numeric part: {}", cost, numeric_part);
        total_complexity += cost * numeric_part;
    }

    println!("----------------");
    println!("total complexity: {}", total_complexity);
}
