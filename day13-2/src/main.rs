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

fn get_candidates(claw_machine: &ClawMachine) -> (usize, usize, usize) {
    println!("{:?}", claw_machine);
    let mut state = 0;
    let mut start_i = 0;
    let mut gap_i = 0;

    for i in 0..claw_machine.target_x / claw_machine.button_a_x + 1 {
        let remain = claw_machine.target_x - i * claw_machine.button_a_x;
        if remain % claw_machine.button_b_x == 0 {
            if state == 0 {
                start_i = i;
                state = 1;
            } else if state == 1 {
                gap_i = i - start_i;
                break;
            }
        }
        if i > 4944808 {
            break;
        }
    }

    if state == 0 {
        return (0, 0, 0);
    }

    if gap_i == 0 {
        return (start_i, 0, 1);
    }

    let count_i =
        (claw_machine.target_x / claw_machine.button_a_x - start_i) / gap_i + 1;
    (start_i, gap_i, count_i)
}

fn get_minimum_token(
    claw_machine: &ClawMachine,
    candidates: (usize, usize, usize),
    start: usize,
    end: usize,
) -> Option<usize> {
    let start_i = candidates.0;
    let gap_i = candidates.1;
    let count_i = candidates.2;
    let estimate_y = |index: usize| -> usize {
        let i = start_i + gap_i * index;
        let remain = claw_machine.target_x - i * claw_machine.button_a_x;
        let j = remain / claw_machine.button_b_x;
        let y = i * claw_machine.button_a_y + j * claw_machine.button_b_y;
        y
    };
    let gap = estimate_y(1) as isize - estimate_y(0) as isize;

    if count_i == 0 || start >= end {
        return None;
    }

    let mid = (start + end) / 2;
    let i = start_i + gap_i * mid;

    let remain = claw_machine.target_x - i * claw_machine.button_a_x;
    let j = remain / claw_machine.button_b_x;
    let y = i * claw_machine.button_a_y + j * claw_machine.button_b_y;
    let cost = if y == claw_machine.target_y {
        Some(i * 3 + j)
    } else {
        None
    };
    // println!("(i, j, cost): ({}, {}, {:?})", i, j, cost);
    if count_i == 1 {
        return cost;
    }

    if gap > 0 {
        if y > claw_machine.target_y {
            get_minimum_token(claw_machine, candidates, start, mid).or(cost)
        } else {
            get_minimum_token(claw_machine, candidates, mid + 1, end).or(cost)
        }
    } else {
        if y > claw_machine.target_y {
            get_minimum_token(claw_machine, candidates, mid + 1, end).or(cost)
        } else {
            get_minimum_token(claw_machine, candidates, start, mid).or(cost)
        }
    }
}

fn main() {
    let claw_machines = parse_file();

    let mut count_tokens = 0;
    for cm in &claw_machines {
        let candidates = get_candidates(cm);
        // println!("candidates: {:?}", candidates);
        println!("candidates: {:?}", candidates.2);
        let token = get_minimum_token(cm, candidates, 0, candidates.2)
            .unwrap_or_default();
        println!("token: {}", token);
        count_tokens += token;
    }
    println!("count_tokens: {}", count_tokens);
}
