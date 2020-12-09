use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Acc,
    Nop,
    Jmp,
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    arg: i64,
}

impl From<String> for Instruction {
    fn from(val: String) -> Self {
        let tokens = val.split(" ").collect_vec();
        let arg = tokens[1].parse::<i64>().unwrap();
        let op = match tokens[0] {
            "acc" => Operation::Acc,
            "nop" => Operation::Nop,
            "jmp" => Operation::Jmp,
            _ => panic!("invalid input"),
        };

        Self { arg, op }
    }
}

impl Instruction {
    fn switch_op(&mut self) {
        match self.op {
            Operation::Jmp => self.op = Operation::Nop,
            Operation::Nop => self.op = Operation::Jmp,
            Operation::Acc => {}
        }
    }

    fn is_change_candidate(&self) -> bool {
        self.op == Operation::Jmp || self.op == Operation::Nop
    }
}

#[derive(Default)]
struct Program {
    ip: u64,
    acc: i64,
    executed_instructions: Vec<u64>,
}

impl Program {
    fn execute(&mut self, instr: &Instruction) {
        match instr.op {
            Operation::Acc => {
                self.acc = self.acc + instr.arg;
                self.ip += 1;
            }
            Operation::Nop => {
                self.ip += 1;
            }
            Operation::Jmp => {
                self.ip = (self.ip as i64 + instr.arg) as u64;
            }
        }
    }

    fn loop_detected(&self) -> bool {
        self.executed_instructions.contains(&self.ip)
    }

    fn execute_until_loop(&mut self, instrs: &Vec<Instruction>) {
        while !self.loop_detected() {
            if let Some(instruction) = instrs.get(self.ip as usize) {
                self.executed_instructions.push(self.ip);
                self.execute(instruction);
            } else {
                break;
            }
        }
    }

    fn update_instruction(&mut self, instrs: &mut Vec<Instruction>, index: usize) {
        let instr = instrs.get_mut(index).unwrap();
        instr.switch_op();
    }

    fn revert_update(&mut self, instrs: &mut Vec<Instruction>, index: usize) {
        let instr = instrs.get_mut(index).unwrap();
        instr.switch_op();
    }

    fn reset_program_state(&mut self) {
        self.executed_instructions = Vec::new();
        self.acc = 0;
        self.ip = 0;
    }

    fn update_to_execute_till_end(&mut self, instrs: &mut Vec<Instruction>) {
        // doing a dry-run so we can populated the instruction candidates for changing.
        self.execute_until_loop(instrs);
        let mut change_candidates = self
            .executed_instructions
            .clone()
            .into_iter()
            .filter(|instr_id| {
                instrs
                    .get(*instr_id as usize)
                    .unwrap()
                    .is_change_candidate()
            })
            .collect_vec();

        while (self.ip as usize) <= instrs.len() - 1 {
            let change_candidate = change_candidates.pop().unwrap() as usize;
            self.update_instruction(instrs, change_candidate);

            self.reset_program_state();
            self.execute_until_loop(instrs);

            self.revert_update(instrs, change_candidate);
        }
    }

    fn acc(&self) -> i64 {
        self.acc
    }
}

fn read_input(path: &Path) -> Vec<Instruction> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut res = Vec::new();
    for line in lines {
        res.push(Instruction::from(line.unwrap()));
    }

    res
}

pub fn solve_puzzle(path: &Path) {
    let mut input = read_input(path);
    let mut program = Program::default();

    program.execute_until_loop(&input);
    println!("Accumulator before infinite loop: {}", program.acc());

    program.update_to_execute_till_end(&mut input);
    println!(
        "Accumulator after executing all instructions: {}",
        program.acc()
    );
}

#[test]
fn test_loop() {
    solve_puzzle(Path::new("src/dec8/input.txt"));
}
