use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instructions: Vec<usize>,
    pc: usize,
    outputs: Vec<usize>,
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

    println!("instructions len: {}", instructions.len());
    Computer {
        reg_a,
        reg_b,
        reg_c,
        instructions,
        pc: 0,
        outputs: Vec::new(),
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

    fn step(&mut self) -> bool {
        if self.pc >= self.instructions.len() {
            return false;
        }

        let opcode = self.instructions[self.pc];
        let operand = self.instructions[self.pc + 1];

        match opcode {
            0 => self.reg_a /= 2_usize.pow(self.combo(operand) as u32),
            1 => self.reg_b ^= operand % 8,
            2 => self.reg_b = self.combo(operand) % 8,
            3 if self.reg_a != 0 => self.pc = operand,
            4 => self.reg_b ^= self.reg_c,
            5 => self.outputs.push(self.combo(operand) % 8),
            6 => self.reg_b = self.reg_a / 2_usize.pow(self.combo(operand) as u32),
            7 => self.reg_c = self.reg_a / 2_usize.pow(self.combo(operand) as u32),
            _ => (),
        }

        if !(opcode == 3 && self.reg_a != 0) {
            self.pc += 2;
        }
        true
    }

    fn run(&mut self) {
        loop {
            let run_successfully = self.step();
            if !run_successfully {
                break;
            }
        }
    }

    fn is_same(&self) -> bool {
        // self.instructions == self.outputs
        self.outputs == [2, 4, 1, 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0]
    }
}

fn test(mut computer: Computer, reg_a: usize) -> bool {
    computer.reg_a = reg_a;
    println!("reg_a: {}", computer.reg_a);
    computer.run();
    println!("{:?}", computer.outputs);
    println!("outputs len: {}", computer.outputs.len());
    println!("{:?}", computer.is_same());
    if computer.is_same() {
        return true;
    }
    println!("----------------");
    false
}

fn main() {
    let computer = input();
    println!("{:#?}", computer);

    // [0, 1, 7, 5, 5, 3, 0]
    // 1925192, 1925193, 1925194, 1925199, 1925232, 1925233, 1925237, 1925239...
    let reg_a: usize = 1925192;
    let reg_a: usize = (reg_a * 8 * 8 + 3) * 8 * 8 + 6;
    let reg_a: usize = reg_a * 8;
    let reg_a: usize = reg_a * 8;
    let reg_a: usize = 504677705105;
    let reg_a: usize = reg_a * 8;
    let reg_a: usize = reg_a * 8;
    let reg_a: usize = 32299373126724;
    let reg_a: usize = reg_a * 8;
    let reg_a: usize = 258394985014171;
    // let reg_a: usize = 63084693456;
    // test(computer.clone(), reg_a);
    // return;

    for i in reg_a..reg_a + 1000000 {
        let is_success = test(computer.clone(), i);
        if is_success {
            break;
        }
    }
}
