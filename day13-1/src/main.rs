use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    button_a_x: i32,
    button_a_y: i32,
    button_b_x: i32,
    button_b_y: i32,
    target_x: i32,
    target_y: i32,
}

fn parse_file() -> Vec<ClawMachine> {
    let filename = env::args().nth(1).unwrap();
    let mut claw_machines = Vec::new();
    let mut claw_machine = ClawMachine {
        button_a_x: 0,
        button_a_y: 0,
        button_b_x: 0,
        button_b_y: 0,
        target_x: 0,
        target_y: 0,
    };

    for line in read_to_string(filename).unwrap().lines() {
        if line.starts_with("Button A") {
            if let Some((part_x, part_y)) = line
                .strip_prefix("Button A: X+")
                .unwrap()
                .split_once(", Y+")
            {
                claw_machine.button_a_x = part_x.parse().unwrap();
                claw_machine.button_a_y = part_y.parse().unwrap();
            }
        } else if line.starts_with("Button B") {
            if let Some((part_x, part_y)) = line
                .strip_prefix("Button B: X+")
                .unwrap()
                .split_once(", Y+")
            {
                claw_machine.button_b_x = part_x.parse().unwrap();
                claw_machine.button_b_y = part_y.parse().unwrap();
            }
        } else if line.starts_with("P") {
            if let Some((part_x, part_y)) =
                line.strip_prefix("Prize: X=").unwrap().split_once(", Y=")
            {
                claw_machine.target_x = part_x.parse().unwrap();
                claw_machine.target_y = part_y.parse().unwrap();
                claw_machines.push(claw_machine);
            }
        }
    }

    claw_machines
}

#[derive(Eq, PartialEq)]
struct State {
    count_a: i32,
    count_b: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_cost = self.count_a * 3 + self.count_b;
        let other_cost = other.count_a * 3 + other.count_b;
        self_cost.cmp(&other_cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_cost = self.count_a * 3 + self.count_b;
        let other_cost = other.count_a * 3 + other.count_b;
        Some(self_cost.cmp(&other_cost).reverse())
    }
}

fn get_minimal_token(claw_machine: &ClawMachine) -> Option<i32> {
    println!("{:?}", claw_machine);
    let mut heap = BinaryHeap::new();
    let mut dp = HashMap::new();
    heap.push(State {
        count_a: 0,
        count_b: 0,
    });

    while let Some(State { count_a, count_b }) = heap.pop() {
        if let Some(_) = dp.get(&(count_a, count_b)) {
            continue;
        }
        dp.insert((count_a, count_b), true);

        let px = count_a * claw_machine.button_a_x
            + count_b * claw_machine.button_b_x;
        let py = count_a * claw_machine.button_a_y
            + count_b * claw_machine.button_b_y;
        if px > claw_machine.target_x || py > claw_machine.target_y {
            continue;
        }
        if px == claw_machine.target_x && py == claw_machine.target_y {
            return Some(count_a * 3 + count_b);
        }

        heap.push(State {
            count_a: count_a + 1,
            count_b: count_b,
        });
        heap.push(State {
            count_a: count_a,
            count_b: count_b + 1,
        });
    }

    None
}

fn main() {
    let claw_machines = parse_file();

    let mut count_tokens = 0;
    for cm in &claw_machines {
        count_tokens += get_minimal_token(cm).unwrap_or_default();
    }
    println!("count_tokens: {}", count_tokens);
}
