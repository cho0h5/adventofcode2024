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

// 7 8 9
// 4 5 6
// 1 2 3
//   0 A

//   ^ A
// < v >
type NumericMap = HashMap<(char, char), &'static str>;
fn generate_numeric_map() -> NumericMap {
    let mut map = HashMap::new();

    map.insert(('A', '0'), "<A");
    map.insert(('0', '2'), "^A");
    map.insert(('2', '9'), ">^^A"); // ^^> >^^
    map.insert(('9', 'A'), "vvvA");

    map.insert(('A', '9'), "^^^A");
    map.insert(('9', '8'), "<A");
    map.insert(('8', '0'), "vvvA");
    map.insert(('0', 'A'), ">A");

    map.insert(('A', '1'), "^<<A");
    map.insert(('1', '7'), "^^A");
    map.insert(('7', '9'), ">>A");
    // map.insert(('9', 'A'), "A");

    map.insert(('A', '4'), "^^<A");
    map.insert(('4', '5'), ">A");
    map.insert(('5', '6'), ">A");
    map.insert(('6', 'A'), "vvA");

    map.insert(('A', '3'), "^A");
    map.insert(('3', '7'), "<<^^A"); // <<^^ ^^<<

    map
}

fn resolve_numeric_keypad(numeric_map: &NumericMap, code: &str) -> Vec<String> {
    let mut result = String::new();

    result.push_str(numeric_map[&('A', code.as_bytes()[0] as char)]);
    for i in 0..code.len() - 1 {
        result.push_str(
            numeric_map
                [&(code.as_bytes()[i] as char, code.as_bytes()[i + 1] as char)],
        );
    }

    vec![result]
}

fn resolve_directional_keypad(code: &str) -> String {
    String::from("bar")
}

fn main() {
    let codes = input();
    let numeric_map = generate_numeric_map();

    for code in &codes {
        let resolved1s = resolve_numeric_keypad(&numeric_map, code);
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
