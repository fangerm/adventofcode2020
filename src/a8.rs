use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Machine {
    pc: usize,
    acc: i64,
    code: Vec<Instruction>,
}

impl Machine {
    fn advance(&mut self) -> bool {
        match self.current() {
            Instruction::Nop(_) => self.pc += 1,

            Instruction::Acc(val) => {
                self.acc += val;
                self.pc += 1;
            }

            Instruction::Jmp(val) => self.pc = ((self.pc as i64) + val) as usize,
        }

        self.pc == self.code.len()
    }

    fn current(&self) -> Instruction {
        self.code[self.pc]
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }

    fn new(path: &'static str) -> Machine {
        let code = Instruction::parse_file(path);
        Machine {
            pc: 0,
            acc: 0,
            code,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    /// Do nothing and advance PC by 1, value is ignored
    Nop(i64),
    /// Increment acc by value, advance PC by 1
    Acc(i64),
    /// Add value to PC
    Jmp(i64),
}

impl Instruction {
    fn parse_file(name: &'static str) -> Vec<Instruction> {
        read_to_string(name)
            .expect("Failed to read code")
            .lines()
            .map(Self::make_inst)
            .collect()
    }

    fn make_inst(inst: &str) -> Instruction {
        let mut split = inst.split(' ');
        let inst = split.next().unwrap();
        let num = split.next().unwrap().parse::<i64>().unwrap();
        match inst {
            "nop" => Instruction::Nop(num),
            "acc" => Instruction::Acc(num),
            "jmp" => Instruction::Jmp(num),
            _ => panic!("Unknown instruction"),
        }
    }
}

pub fn a8_1() {
    println!("{}", exec(&mut Machine::new("inputs/input-8")).1);
}

pub fn a8_2() {
    let mut machine = Machine::new("inputs/input-8");
    for i in 0..machine.code.len() {
        let original = machine.code[i];
        machine.code[i] = match original {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            _ => original,
        };

        let (success, acc) = exec(&mut machine);
        if success {
            println!("{}", acc);
            return;
        }

        machine.code[i] = original;
        machine.reset();
    }
}

// Execute the machine until the program exits regularly or due to a repeated
// instruction.
// Returns (exit_normally, acc)
fn exec(machine: &mut Machine) -> (bool, i64) {
    let mut instruction_positions_executed = HashSet::<usize>::new();
    loop {
        let done = machine.advance();
        if done || instruction_positions_executed.contains(&machine.pc) {
            return (done, machine.acc);
        }
        instruction_positions_executed.insert(machine.pc);
    }
}
