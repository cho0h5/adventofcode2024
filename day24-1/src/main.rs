use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

type Gate = HashMap<String, (String, String, String)>;

fn input() -> (HashMap<String, bool>, Gate) {
    let mut wires = HashMap::new();
    let mut gates = HashMap::new();

    let mut flag = false;
    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            flag = true;
            continue;
        }

        if !flag {
            let (wire, value) = line.split_once(": ").unwrap();
            let value: usize = value.parse().unwrap();

            match value {
                0 => wires.insert(wire.to_owned(), false),
                1 => wires.insert(wire.to_owned(), true),
                _ => panic!("no way"),
            };
        } else {
            let (part_x, out) = line.split_once(" -> ").unwrap();
            let mut part_x_splited = part_x.split(' ');
            let in1 = part_x_splited.next().unwrap().to_owned();
            let logic = part_x_splited.next().unwrap().to_owned();
            let in2 = part_x_splited.next().unwrap().to_owned();
            gates.insert(out.to_owned(), (in1, logic, in2));
        }
    }

    (wires, gates)
}

fn analysis(
    wires: &mut HashMap<String, bool>,
    gates: &Gate,
    target: &str,
) -> bool {
    if wires.contains_key(target) {
        return wires[target];
    }
    let (in1, logic, in2) = &gates[target];

    let in1_value = analysis(wires, gates, in1);
    let in2_value = analysis(wires, gates, in2);

    match logic.as_str() {
        "OR" => wires.insert(target.to_owned(), in1_value | in2_value),
        "AND" => wires.insert(target.to_owned(), in1_value & in2_value),
        "XOR" => wires.insert(target.to_owned(), in1_value ^ in2_value),
        _ => None,
    };
    wires[target]
}

fn main() {
    let (mut wires, gates) = input();
    // println!("wires: {:?}", wires);
    // println!("gates: {:?}", gates);

    for wire in gates.keys() {
        analysis(&mut wires, &gates, wire);
    }

    let mut z_keys: Vec<&String> =
        wires.keys().filter(|k| k.starts_with('z')).collect();
    z_keys.sort();

    let mut result: usize = 0;
    for k in z_keys {
        let digit: usize = k[1..].parse().unwrap();
        let v = wires[k];
        result |= if v { 1 << digit } else { 0 }
    }
    println!("result: {}", result);
}
