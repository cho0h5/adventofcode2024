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

    map.insert(('A', '4'), "^^<<A");
    map.insert(('4', '5'), ">A");
    map.insert(('5', '6'), ">A");
    map.insert(('6', 'A'), "vvA");

    map.insert(('A', '3'), "^A");
    map.insert(('3', '7'), "<<^^A"); // <<^^ ^^<<

    //

    map.insert(('A', '8'), "<^^^A"); // <^^^ ^^^<
    map.insert(('8', '3'), ">vvA"); // >vv vv>
    map.insert(('3', '9'), "^^A");

    map.insert(('1', '6'), "^>>A"); // >>^ ^>>
    map.insert(('6', '9'), "^A");

    map.insert(('A', '5'), "<^^A"); // <^^ ^^<
    map.insert(('5', '7'), "<^A"); // <^ ^<

    map.insert(('A', '6'), "^^A");
    map.insert(('6', '7'), "^<<A"); // ^<< <<^
    map.insert(('7', '0'), ">vvvA");

    map.insert(('6', '3'), "vA");
    map.insert(('3', '8'), "<^^A"); // <^^ ^^<
    map.insert(('8', 'A'), ">vvvA"); // >vvv vvv>

    map
}

//   ^ A
// < v >
type DirectionalMap = HashMap<(char, char), &'static str>;
fn generate_directional_map() -> DirectionalMap {
    let mut map = HashMap::new();

    map.insert(('A', 'A'), "A");
    map.insert(('A', '^'), "<A");
    map.insert(('A', '<'), "v<<A");
    map.insert(('A', 'v'), "<vA"); // v< 324 <v 423
    map.insert(('A', '>'), "vA");

    map.insert(('^', 'A'), ">A");
    map.insert(('^', '^'), "A");
    map.insert(('^', '<'), "v<A");
    map.insert(('^', 'v'), "vA");
    map.insert(('^', '>'), ">vA"); // >v vt

    map.insert(('<', 'A'), ">>^A");
    map.insert(('<', '^'), ">^A");
    map.insert(('<', '<'), "A");
    map.insert(('<', 'v'), ">A");
    map.insert(('<', '>'), ">>A");

    map.insert(('v', 'A'), ">^A"); // >^ ^>
    map.insert(('v', '^'), "^A");
    map.insert(('v', '<'), "<A");
    map.insert(('v', 'v'), "A");
    map.insert(('v', '>'), ">A");

    map.insert(('>', 'A'), "^A");
    map.insert(('>', '^'), "<^A"); // <^ ^>
    map.insert(('>', '<'), "<<A");
    map.insert(('>', 'v'), "<A");
    map.insert(('>', '>'), "A");

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

fn resolve_directional_keypad(
    directional_map: &DirectionalMap,
    code: &str,
) -> String {
    let mut result = String::new();

    result.push_str(directional_map[&('A', code.as_bytes()[0] as char)]);
    for i in 0..code.len() - 1 {
        result.push_str(
            directional_map
                [&(code.as_bytes()[i] as char, code.as_bytes()[i + 1] as char)],
        );
    }

    result
}

fn main() {
    let codes = input();
    let numeric_map = generate_numeric_map();
    let directional_map = generate_directional_map();

    let mut sum = 0;
    for code in &codes {
        let resolved1s = resolve_numeric_keypad(&numeric_map, code);
        println!("----------------");
        println!("code: {}", code);
        for resolved1 in resolved1s {
            let resolved2 =
                resolve_directional_keypad(&directional_map, &resolved1);
            let resolved3 =
                resolve_directional_keypad(&directional_map, &resolved2);
            println!("resolved1: {}", resolved1);
            println!("resolved2: {}", resolved2);
            println!("resolved3: {}", resolved3);

            let len = resolved3.len();
            let numeric_part =
                code[0..code.len() - 1].parse::<usize>().unwrap();
            let complexity = len * numeric_part;
            println!(
                "len: {}, numeric_part: {}, complexity: {}",
                len, numeric_part, complexity
            );
            sum += complexity;
        }
    }
    println!("sum: {}", sum);
}
