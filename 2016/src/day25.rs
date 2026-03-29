use crate::day12::Register;
use crate::helper;

// change for day 25: values > i8 range, so change the data type everywhere
type Word = i16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Cpyi {val: Word, dst: Register},
    Cpyr {src: Register, dst: Register},
    Inc {dst: Register},
    Dec {dst: Register},
    Jnzr {src: Register, off: Word},
    Jnzi {val: Word, off: Word},  // jnz with fixed value
    // new for day 25
    Outr {src: Register}
}
impl Instruction {
    fn parse(s: &str) -> Instruction {
        let words = s.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "cpy" => {
                let val= words[1].parse::<Word>();
                match val {
                    Ok(val) => Instruction::Cpyi {val, dst: Register::from_str(words[2])},
                    Err(_) => Instruction::Cpyr {src: Register::from_str(words[1]), dst: Register::from_str(words[2])}
                }
            }
            "inc" => Instruction::Inc {dst: Register::from_str(words[1])},
            "dec" => Instruction::Dec {dst: Register::from_str(words[1])},
            "jnz" => {
                let val= words[1].parse::<Word>();
                let off = words[2].parse::<Word>().unwrap();
                match val {
                    Ok(val) => Instruction::Jnzi {val, off},
                    Err(_) => Instruction::Jnzr {src: Register::from_str(words[1]), off}
                }
            }
            "out" => Instruction::Outr {src: Register::from_str(words[1])},
            _ => panic!("invalid instruction")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Computer {
    registers: [i32; 4],
    pc: Word,
    instructions: Vec<Instruction>,
}
impl Computer {
    fn new(input_text: String) -> Computer {
        let instructions: Vec<Instruction> = input_text.lines().map(Instruction::parse).collect();
        Computer{
            registers: [0,0,0,0],
            pc: 0,
            instructions,
        }
    }
    // now with optional output value
    fn execute_one_instruction(&mut self) -> Option<i32> {
        let instruction = self.instructions[self.pc as usize];
        match instruction {
            Instruction::Cpyi {val, dst} => {
                self.registers[dst as usize] = val as i32;
                self.pc += 1;
            }
            Instruction::Cpyr {src, dst} => {
                self.registers[dst as usize] = self.registers[src as usize];
                self.pc += 1;
            }
            Instruction::Inc {dst} => {
                self.registers[dst as usize] += 1;
                self.pc += 1;
            }
            Instruction::Dec {dst} => {
                self.registers[dst as usize] -= 1;
                self.pc += 1;
            }
            Instruction::Jnzr {src, off} => {
                if self.registers[src as usize] != 0 {
                    self.pc += off;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnzi {val, off} => {
                if val != 0 {
                    self.pc += off;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Outr {src} => {
                self.pc += 1;
                return Some(self.registers[src as usize]);
            }
        }
        None
    }
    // run until end of program, or until number of output values collected
    fn execute(&mut self, wait_for_output_len: usize) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::with_capacity(wait_for_output_len);
        while self.pc >= 0 && self.pc < self.instructions.len() as Word && output.len() < wait_for_output_len {
            match self.execute_one_instruction() {
                Some(val) => output.push(val),
                None => {}
            }
        }
        output
    }
}

// core function: run every computer as long as the output matches the expected output
// since we cannot verify 'forever' we only check the given number of output values
fn find_reg_with_pattern(computer: &Computer, check_length: u16) -> i32 {
    let mut value_a: i32 = 0;

    'outer: loop {
        let mut c = computer.clone();
        c.registers[Register::A as usize] = value_a;

        let mut checked_output_values: u16 = 0;

        while checked_output_values <= check_length {
            match c.execute_one_instruction() {
                Some(val) => {
                    let expected = if (checked_output_values % 2) == 0 { 0 } else { 1 };
                    if val != expected {
                        value_a += 1;
                        continue 'outer;  // wrong output, next computer
                    } else {
                        checked_output_values += 1;
                    }
                }
                None => {}
            }
        }
        return value_a;
    }
}

fn solve_part1(input_text: String) -> String {
    let computer = Computer::new(input_text);
    find_reg_with_pattern(&computer, 32).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 25)
            .expect("Could not fetch input");
    println!("Part 1: {}", solve_part1(input_text));
}
