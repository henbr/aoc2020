use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Clone)]
struct Machine {
    program: Vec<Instruction>,
    pc: i32,
    acc: i32,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        let pc = self.pc as usize;
        if pc >= self.program.len() {
            panic!("Error in program");
        }
        match self.program[pc] {
            Instruction::Nop(_) => {}
            Instruction::Acc(a) => self.acc += a,
            Instruction::Jmp(offset) => self.pc += offset - 1,
        }
        self.pc += 1;
    }
}

fn load_program(file: &str) -> Vec<Instruction> {
    let data = fs::read_to_string(file).unwrap();
    data.lines()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<&str>>();
            if parts.len() > 0 {
                let inst = parts[0];
                let num = parts[1].parse::<i32>().unwrap();
                match inst {
                    "nop" => Instruction::Nop(num),
                    "acc" => Instruction::Acc(num),
                    "jmp" => Instruction::Jmp(num),
                    _ => panic!("Unknown instruction {}", inst),
                }
            } else {
                panic!("Error in program: {}", l);
            }
        })
        .collect()
}

fn part1(file: &str) {
    let program = load_program(file);
    let mut visited = vec![false; program.len()];
    let mut machine = Machine::new(program);
    loop {
        let pc = machine.pc as usize;
        if pc >= machine.program.len() {
            panic!("Error in program");
        }
        if visited[pc] {
            break;
        }
        visited[pc] = true;
        machine.step();
    }
    println!("Part 1: {}", machine.acc);
}

fn part2(file: &str) {
    let program = load_program(file);
    let machine = Machine::new(program);
    let mut acc = None;
    'outer: for idx in 0..machine.program.len() {
        let mut machine = machine.clone();
        match machine.program[idx] {
            Instruction::Nop(a) => machine.program[idx] = Instruction::Jmp(a),
            Instruction::Acc(_) => continue,
            Instruction::Jmp(a) => machine.program[idx] = Instruction::Nop(a),
        }

        let mut visited = vec![false; machine.program.len()];
        'inner: loop {
            let pc = machine.pc as usize;
            if pc >= machine.program.len() {
                acc = Some(machine.acc);
                break 'outer;
            }
            if visited[pc] {
                break 'inner;
            }
            visited[pc] = true;
            machine.step();
        }
    }
    if let Some(acc) = acc {
        println!("Part 2: {}", acc);
    } else {
        println!("Part 2: end not found");
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Reading file: {}", args[1]);
        part1(&args[1]);
        part2(&args[1]);
    } else {
        println!("No input file specified");
    }
}
