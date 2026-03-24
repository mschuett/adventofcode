use crate::helper;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Register {
    A = 0, B, C, D
}
impl Register {
    fn from_str(s: &str) -> Register {
        assert_eq!(s.len(), 1);
        let c = s.chars().next().unwrap();
        match c {
            'a' => Register::A,
            'b' => Register::B,
            'c' => Register::C,
            'd' => Register::D,
            _ => panic!("invalid register")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Cpyi {val: i8, dst: Register},
    Cpyr {src: Register, dst: Register},
    Inc {dst: Register},
    Dec {dst: Register},
    Jnzr {src: Register, off: i8},
    Jnzi {val: i8, off: i8},  // jnz with fixed value
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
            }
            "inc" => Instruction::Inc {dst: Register::from_str(words[1])},
            "dec" => Instruction::Dec {dst: Register::from_str(words[1])},
            "jnz" => {
                let val= words[1].parse::<i8>();
                let off = words[2].parse::<i8>().unwrap();
                match val {
                    Ok(val) => Instruction::Jnzi {val, off},
                    Err(_) => Instruction::Jnzr {src: Register::from_str(words[1]), off}
                }
            }
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
        }
    }
    fn execute(&mut self) {
        while self.pc >= 0 && self.pc < self.instructions.len() as i8 {
            self.execute_one_instruction()
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let mut comp = Computer::new(input_text);
    comp.execute();
    comp.registers[Register::A as usize].to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut comp = Computer::new(input_text);
    comp.registers[Register::C as usize] = 1;
    comp.execute();
    comp.registers[Register::A as usize].to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 12)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "42");
    }
}
