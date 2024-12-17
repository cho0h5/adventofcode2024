use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instructions: Vec<usize>,
    pc: usize,
}

fn input() -> Computer {
    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut instructions = Vec::new();

    let filename = env::args().nth(1).unwrap();
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        match i {
            0 => {
                reg_a = line
                    .strip_prefix("Register A: ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            }
            1 => {
                reg_b = line
                    .strip_prefix("Register B: ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            }
            2 => {
                reg_c = line
                    .strip_prefix("Register C: ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            }
            4 => {
                instructions.extend(
                    line.strip_prefix("Program: ")
                        .unwrap()
                        .split(',')
                        .map(|c| c.parse::<usize>().unwrap()),
                );
            }
            _ => (),
        }
    }

    Computer {
        reg_a,
        reg_b,
        reg_c,
        instructions,
        pc: 0,
    }
}

impl Computer {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            x if x <= 3 => x,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("invalid combo operand: {}", operand),
        }
    }

    fn run(&mut self) -> bool {
        if self.pc >= self.instructions.len() {
            return false;
        }

        let opcode = self.instructions[self.pc];
        let operand = self.instructions[self.pc + 1];
        // println!("{:?}", self);
        // println!("opcode: {}, operand: {}", opcode, operand);

        match opcode {
            0 => self.reg_a /= 2_usize.pow(self.combo(operand) as u32),
            1 => self.reg_b = operand % 8,
            2 => self.reg_b ^= self.combo(operand) % 8,
            3 if self.reg_a != 0 => self.pc = operand,
            4 => self.reg_b ^= self.reg_c,
            5 => print!("{},", self.combo(operand) % 8),
            6 => self.reg_b = self.reg_a / 2_usize.pow(self.combo(operand) as u32),
            7 => self.reg_c = self.reg_a / 2_usize.pow(self.combo(operand) as u32),
            _ => (),
        }

        if !(opcode == 3 && self.reg_a != 0) {
            self.pc += 2;
        }
        true
    }
}

fn main() {
    let mut computer = input();
    println!("{:#?}", computer);

    loop {
        let run_successfully = computer.run();
        if !run_successfully {
            break;
        }
    }
}
