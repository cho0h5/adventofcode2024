use std::cmp::Ordering;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    button_a_x: usize,
    button_a_y: usize,
    button_b_x: usize,
    button_b_y: usize,
    target_x: usize,
    target_y: usize,
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
                // claw_machine.target_x = part_x.parse::<usize>().unwrap();
                // claw_machine.target_y = part_y.parse::<usize>().unwrap();
                claw_machine.target_x =
                    part_x.parse::<usize>().unwrap() + 10000000000000;
                claw_machine.target_y =
                    part_y.parse::<usize>().unwrap() + 10000000000000;
                claw_machines.push(claw_machine);
            }
        }
    }

    claw_machines
}

#[derive(Eq, PartialEq)]
struct State {
    count_a: usize,
    count_b: usize,
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

fn get_minimal_token(claw_machine: &ClawMachine) -> Option<usize> {
    println!("{:?}", claw_machine);
    let mut state = 0;
    let mut start_i = 0;
    let mut gap_i = 0;

    for i in 0..claw_machine.target_x / claw_machine.button_a_x + 1 {
        let remain = claw_machine.target_x - i * claw_machine.button_a_x;
        if remain % claw_machine.button_b_x == 0 {
            // println!("{}, {}", i, remain / claw_machine.button_b_x);
            if state == 0 {
                start_i = i;
                state = 1;
            } else if state == 1 {
                gap_i = i - start_i;
                break;
            }
        }
    }

    if state == 0 {
        return None;
    }

    if gap_i == 0 {
        let i = start_i;
        let remain = claw_machine.target_x - i * claw_machine.button_a_x;
        if remain % claw_machine.button_b_x == 0 {
            let j = remain / claw_machine.button_b_x;
            let y = i * claw_machine.button_a_y + j * claw_machine.button_b_y;
            if y == claw_machine.target_y {
                println!("{}, {}", i, j);
                let cost = 3 * i + j;
                return Some(cost);
            }
        }
        return None;
    }

    let mut min = usize::MAX;
    for i in (start_i..=claw_machine.target_x / claw_machine.button_a_x)
        .step_by(gap_i)
    {
        let remain = claw_machine.target_x - i * claw_machine.button_a_x;
        if remain % claw_machine.button_b_x == 0 {
            let j = remain / claw_machine.button_b_x;
            let y = i * claw_machine.button_a_y + j * claw_machine.button_b_y;
            if y == claw_machine.target_y {
                println!("{}, {}", i, j);
                let cost = 3 * i + j;
                min = std::cmp::min(min, cost);
            }
        }
    }

    if min == usize::MAX {
        None
    } else {
        Some(min)
    }
}

fn main() {
    let claw_machines = parse_file();

    let mut count_tokens = 0;
    for cm in &claw_machines {
        count_tokens += get_minimal_token(cm).unwrap_or_default();
    }
    println!("count_tokens: {}", count_tokens);
}
