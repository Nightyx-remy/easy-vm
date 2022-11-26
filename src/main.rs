use std::fmt::Display;

const STACK_SIZE: usize = 128;

#[derive(Clone, Debug)]
pub enum Instruction {
    Push(u8),
    PushStr(&'static str),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    JmpEq(usize),
    JmpNeq(usize),
    Jmp(usize),
    StdCall(usize),
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

pub enum StdFunc {
    PrintU8 = 0x0,
    PrintChar = 0x1,
    PrintString = 0x2,
    Clone = 0x3
}

pub struct VM {
    stack: [u8; STACK_SIZE],
    stack_pointer: usize,
    program_pointer: usize,
    overflow: bool,
}

impl VM {

    pub fn new() -> VM {
        Self {
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            program_pointer: 0,
            overflow: false,
        }
    }

    pub fn execute(&mut self, program: &Program, debug: bool) {
        loop {
            if debug {
                println!("Instruction: {:?}", program.get(self.program_pointer));   
            }
            if !self.execute_one(program) {
                break;
            }
            if debug {
                println!("{}\n", self);
            }
        }
    }

    fn stack_push(&mut self, value: u8) {
        if self.stack_pointer >= STACK_SIZE {
            panic!("Stack Overflow!");
        }
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    fn stack_pop(&mut self) -> u8 {
        if self.stack_pointer == 0 {
            panic!("Stack Underflow!");
        }
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }

    pub fn execute_one(&mut self, program: &Program) -> bool {
        let instruction = program.get(self.program_pointer);

        match instruction {
            Instruction::Push(value) => self.stack_push(value),
            Instruction::PushStr(value) => {
                self.stack_push(0);
                for chr in value.chars().rev() {
                    self.stack_push(chr as u8);
                }
            },
            Instruction::Pop => _ = self.stack_pop(),
            Instruction::Add => {
                let lhs = self.stack_pop();
                let rhs = self.stack_pop();
                let (value, overflow) = lhs.overflowing_add(rhs);
                self.stack_push(value);
                self.overflow = overflow;
            },
            Instruction::Sub => {
                let lhs = self.stack_pop();
                let rhs = self.stack_pop();
                let (value, overflow) = lhs.overflowing_sub(rhs);
                self.stack_push(value);
                self.overflow = overflow;
            },
            Instruction::Mul => {
                let lhs = self.stack_pop();
                let rhs = self.stack_pop();
                self.stack_push(lhs * rhs);
            },
            Instruction::Div => {
                let lhs = self.stack_pop();
                let rhs = self.stack_pop();
                self.stack_push(lhs / rhs);
            },
            Instruction::JmpEq(location) => {
                let lhs = self.stack_pop();
                let rhs = self.stack_pop();
                if lhs == rhs {
                    self.program_pointer = location;
                    // Push the values back once compared
                    self.stack_push(rhs);
                    self.stack_push(lhs);
                    return true;
                } else {
                    // Push the values back once compared
                    self.stack_push(rhs);
                    self.stack_push(lhs);
                }
            },
            Instruction::JmpNeq(location) => {
                let lhs = self.stack_pop();
                let rhs = self.stack_pop();
                if lhs != rhs {
                    self.program_pointer = location;
                    // Push the values back once compared
                    self.stack_push(rhs);
                    self.stack_push(lhs);
                    return true;
                } else {
                    // Push the values back once compared
                    self.stack_push(rhs);
                    self.stack_push(lhs);
                }
            },
            Instruction::Jmp(location) => {
                self.program_pointer = location;
                return true;
            },
            Instruction::StdCall(id) => {
                unsafe {
                    let func: StdFunc = std::mem::transmute(id as u8);
                    match func {
                        StdFunc::PrintU8 => {
                            let value = self.stack_pop();
                            print!("{}", value);
                        },
                        StdFunc::PrintChar => {
                            let value = self.stack_pop() as char;
                            print!("{}", value);
                        },
                        StdFunc::PrintString => {
                            while self.stack_pointer > 0 {
                                let value = self.stack_pop() as char;
                                if value == '\0' {
                                    break;
                                }
                                print!("{}", value);
                            }
                        },
                        StdFunc::Clone => {
                            self.stack_push(self.stack[self.stack_pointer]);
                        },
                    }
                }
            },
            Instruction::Interupt => return false,
        }
        self.program_pointer += 1;

        return true;
    }

}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program Pointer: {}\n", self.program_pointer)?;
        write!(f, "Stack [{}]:\n", self.stack_pointer)?;
        'A: for i in 0..(STACK_SIZE / 32) {
            for j in 0..32 {
                let index = i * 8 + j;
                if index >= self.stack_pointer {
                    break 'A;
                }
                write!(f, "{:02x} ", self.stack[index])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut program = Program::new();
    // Loop [0; 10[
    // Equivalent program:
    // i = 0
    // while i < 10 {
    //      i += 1;   
    // }
    // program.push(Instruction::Push(10));  // 0
    // program.push(Instruction::Push(0));   // 1
    // program.push(Instruction::JmpEq(6));  // 2
    // program.push(Instruction::Push(1));   // 3
    // program.push(Instruction::Add);       // 4
    // program.push(Instruction::Jmp(2));    // 5

    program.push(Instruction::PushStr("Hello, World!\n"));
    program.push(Instruction::StdCall(StdFunc::PrintString as usize));

    let mut vm = VM::new();
    vm.execute(&program, false);
    // println!("{}", vm);
}
