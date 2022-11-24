use std::{ops::Index, fmt::Display};

#[derive(Clone)]
pub enum Instruction {
    PushReg {
        register: usize,
        data: u8
    },
    Add {
        lhs: usize,
        rhs: usize,
        res: usize,
    },
    PushStack {
        register: usize
    },
    PopStack {
        register: usize
    },
    Interupt
}

pub struct Program {
    instructions: Vec<Instruction>
}

impl Program {

    pub fn new() -> Self {
        Self {
            instructions: Vec::new()
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn get(&self, index: usize) -> Instruction {
        return self.instructions.get(index).cloned().unwrap_or(Instruction::Interupt);
    }

}

pub struct VM {
    stack: [u8; 1024],
    stack_pointer: usize,
    program_pointer: usize,
    registers: [u8; 8]
}

impl VM {

    pub fn new() -> VM {
        Self {
            stack: [0; 1024],
            stack_pointer: 0,
            program_pointer: 0,
            registers: [0; 8],
        }
    }

    pub fn execute(&mut self, program: Program) {
        loop {
            let instruction = program.get(self.program_pointer);

            match instruction {
                Instruction::PushReg { register, data } => {
                    self.registers[register] = data;
                },
                Instruction::Add { lhs, rhs, res } => {
                    self.registers[res] = self.registers[lhs] + self.registers[rhs];
                },
                Instruction::PushStack { register } => {
                    self.stack[self.stack_pointer] = self.registers[register];
                    self.stack_pointer += 1;
                },
                Instruction::PopStack { register } => {
                    if self.stack_pointer == 0 {
                        panic!("Cannot pop!");
                    }
                    self.registers[register] = self.stack[self.stack_pointer - 1];
                    self.stack_pointer -= 1;
                },
                Instruction::Interupt => break,
            }

            self.program_pointer += 1;
        }
    }

    pub fn execute_one(&mut self, program: &Program) {
        let instruction = program.get(self.program_pointer);

        match instruction {
            Instruction::PushReg { register, data } => {
                self.registers[register] = data;
            },
            Instruction::Add { lhs, rhs, res } => {
                self.registers[res] = self.registers[lhs] + self.registers[rhs];
            },
            Instruction::PushStack { register } => {
                self.stack[self.stack_pointer] = self.registers[register];
                self.stack_pointer += 1;
            },
            Instruction::PopStack { register } => {
                if self.stack_pointer == 0 {
                    panic!("Cannot pop!");
                }
                self.registers[register] = self.stack[self.stack_pointer - 1];
                self.stack_pointer -= 1;
            },
            Instruction::Interupt => {},
        }

        self.program_pointer += 1;
    }

}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stack:\n")?;
        for i in 0..(1024 / 32) {
            for j in 0..32 {
                let index = i * 8 + j;
                write!(f, "{} ", self.stack[index])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\nRegisters:\n")?;
        for i in 0..8 {
            write!(f, "Register[{}]: {}\n", i, self.registers[i])?;
        }
        Ok(())
    }
}

fn main() {
    let mut program = Program::new();
    program.push(Instruction::PushReg { register: 0, data: 2 });
    program.push(Instruction::PushReg { register: 1, data: 3 });
    program.push(Instruction::Add { lhs: 0, rhs: 1, res: 1 });
    program.push(Instruction::PushStack { register: 1 });
    program.push(Instruction::PushReg { register: 1, data: 3 });
    program.push(Instruction::PopStack { register: 0 });
    program.push(Instruction::Add { lhs: 0, rhs: 1, res: 1 });

    let mut vm = VM::new();
    vm.execute(program);
    println!("{}", vm);

    println!("Hello, world!");
}
