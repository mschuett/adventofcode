use crate::helper;
use crate::day12::Register;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Cpyi {val: i8, dst: Register},
    Cpyr {src: Register, dst: Register},
    Inc {dst: Register},
    Dec {dst: Register},
    Jnzr {src: Register, off: i8},
    Jnzi {val: i8, off: i8},  // jnz with fixed value
    // new for day 23
    Tgl {dst: Register},
    // implicitly, because we can toggle to these
    Jnzrr {src: Register, off: Register},
    Jnzir {val: i8, off: Register},
    Nop, // for invalid instructions
}
impl Instruction {
    fn parse(s: &str) -> Instruction {
        let words = s.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "cpy" => {
                let val= words[1].parse::<i8>();
                match val {
                    Ok(val) => Instruction::Cpyi {val, dst: Register::from_str(words[2])},
                    Err(_) => Instruction::Cpyr {src: Register::from_str(words[1]), dst: Register::from_str(words[2])}
                }
            },
            "inc" => Instruction::Inc {dst: Register::from_str(words[1])},
            "dec" => Instruction::Dec {dst: Register::from_str(words[1])},
            "jnz" => {
                let val= words[1].parse::<i8>();
                let off = words[2].parse::<i8>();
                match val {
                    Ok(val) => {
                        match off {
                            Ok(off) => Instruction::Jnzi {val, off},
                            Err(_) => Instruction::Jnzir {val, off: Register::from_str(words[2])}
                        }
                    },
                    Err(_) => {
                        match off {
                            Ok(off) => Instruction::Jnzr {src: Register::from_str(words[1]), off},
                            Err(_) => Instruction::Jnzrr {src: Register::from_str(words[1]), off: Register::from_str(words[2])}
                        }
                    }
                }
            },
            "tgl" => Instruction::Tgl {dst: Register::from_str(words[1])},
            _ => panic!("invalid instruction")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Computer {
    registers: [i32; 4],
    pc: i8,
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
    fn execute_one_instruction(&mut self) {
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
            Instruction::Jnzrr {src, off} => {
                if self.registers[src as usize] != 0 {
                    self.pc += self.registers[off as usize] as i8;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnzir {val, off} => {
                if val != 0 {
                    self.pc += self.registers[off as usize] as i8;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Nop => {
                self.pc += 1;
            }
            Instruction::Tgl { dst: tgl_dst } => {
                let target_pc = self.pc + self.registers[tgl_dst as usize] as i8;
                if target_pc >= 0 && target_pc < self.instructions.len() as i8 {
                    let target_i = self.instructions[target_pc as usize];
                    self.instructions[target_pc as usize] = match target_i {
                        // one argument
                        Instruction::Inc {dst} => Instruction::Dec {dst},
                        Instruction::Dec {dst} => Instruction::Inc {dst},
                        Instruction::Tgl {dst} => Instruction::Inc {dst},
                        // two argument
                        Instruction::Jnzi {val: _, off: _} => Instruction::Nop,
                        Instruction::Jnzr {src: _, off: _} => Instruction::Nop,
                        Instruction::Jnzir {val, off} => Instruction::Cpyi {val, dst: off},
                        Instruction::Jnzrr {src, off} => Instruction::Cpyr {src, dst: off},
                        Instruction::Cpyi {val, dst} => Instruction::Jnzir {val, off: dst},
                        Instruction::Cpyr {src, dst} => Instruction::Jnzrr {src, off: dst},
                        Instruction::Nop => {panic!("trying to toggle an invalid instruction")},
                    }
                }
                self.pc += 1;
            }
        }
    }
    fn execute(&mut self) {
        while self.pc >= 0 && self.pc < self.instructions.len() as i8 {
            self.execute_one_instruction()
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let mut computer = Computer::new(input_text);
    computer.registers[Register::A as usize] = 7;
    computer.execute();
    computer.registers[0].to_string()
}

fn solve_part2(input_text: String) -> String {
    // brute force, not optimized to 'multiply'
    let mut computer = Computer::new(input_text);
    computer.registers[Register::A as usize] = 12;
    computer.execute();
    computer.registers[0].to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 23)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "3");
    }
}
