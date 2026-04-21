use std::collections::VecDeque;
use crate::helper;

// re-using lots of structure from AoC 2016, day 12
type Word = i64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Register {
    A = 0, B, F, I, P
}
impl TryFrom<&str> for Register {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        assert_eq!(s.len(), 1);
        let c = s.chars().next().unwrap();
        match c {
            'a' => Ok(Register::A),
            'b' => Ok(Register::B),
            'f' => Ok(Register::F),
            'i' => Ok(Register::I),
            'p' => Ok(Register::P),
            _ => Err(format!("Unknown register {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// generalisation of Register and Word
enum Param {
    Reg(Register),
    Val(Word)
}
impl TryFrom<&str> for Param {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() == 1 && s.as_bytes()[0].is_ascii_alphabetic() {
            let r = Register::try_from(s);
            match r {
                Ok(r) => Ok(Param::Reg(r)),
                Err(e) => Err(e)
            }
        } else {
            let num = s.parse::<Word>();
            match num {
                Ok(r) => Ok(Param::Val(r)),
                Err(e) => Err(e.to_string())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Snd(Register),
    Set(Register, Param),
    Add(Register, Param),
    Mul(Register, Param),
    Mod(Register, Param),
    Rcv(Register),
    Jgz(Param, Param),
}
impl TryFrom<&str> for Instruction {
    type Error = String;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let words = line.split_whitespace().collect::<Vec<_>>();

        match words[0] {
            "snd" => Ok(Self::Snd(Register::try_from(words[1])?)),
            "set" => Ok(Self::Set(Register::try_from(words[1])?, Param::try_from(words[2])?)),
            "add" => Ok(Self::Add(Register::try_from(words[1])?, Param::try_from(words[2])?)),
            "mul" => Ok(Self::Mul(Register::try_from(words[1])?, Param::try_from(words[2])?)),
            "mod" => Ok(Self::Mod(Register::try_from(words[1])?, Param::try_from(words[2])?)),
            "rcv" => Ok(Self::Rcv(Register::try_from(words[1])?)),
            "jgz" => Ok(Self::Jgz(Param::try_from(words[1])?, Param::try_from(words[2])?)),
            _ => Err(format!("Unknown instruction {}", line)),
        }
    }
}

// basically Interrupts, signalling why an execute_one_instruction()/exec() stopped:
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExecBreakReason {
    EndOfProgram,
    RcvWait,
}

#[derive(Clone, Debug)]
struct Computer {
    registers: [Word; 5],  // with nightly API: [i32; mem::variant_count::<Register>()]
    pc: i8,
    instructions: Vec<Instruction>,
    snd_output: VecDeque<Word>,
    rcv_queue: VecDeque<Word>,
    part2_rcv: bool,  // feature flag for different rcv semantic
    snd_counter: u32,
}
impl Computer {
    fn new(input_text: String, part2_rcv: bool) -> Computer {
        let instructions: Vec<Instruction> = input_text.lines()
            .map(Instruction::try_from)
            .collect::<Result<_, _>>()
            .unwrap();
        Computer {
            registers: [0, 0, 0, 0, 0],
            pc: 0,
            instructions,
            snd_output: VecDeque::new(),
            rcv_queue: VecDeque::new(),
            part2_rcv,
            snd_counter: 0,
        }
    }
    fn execute_one_instruction(&mut self) -> Result<(), ExecBreakReason> {
        // only Part 2: result=Err indicates that we await RCV data
        let instruction = self.instructions[self.pc as usize];
        self.pc += 1;
        match instruction {
            Instruction::Snd(x) => {
                self.snd_output.push_back(self.registers[x as usize]);
                self.snd_counter += 1;
            }
            Instruction::Set(x, y) => {
                let value = match y {
                    Param::Reg(reg_y) => self.registers[reg_y as usize],
                    Param::Val(val_y) => val_y,
                };
                self.registers[x as usize] = value;
            }
            Instruction::Add(x, y) => {
                match y {
                    Param::Reg(reg_y) => self.registers[x as usize] += self.registers[reg_y as usize],
                    Param::Val(val_y)   => self.registers[x as usize] += val_y
                }
            }
            Instruction::Mul(x, y) => {
                let factor = match y {
                    Param::Reg(reg_y) => self.registers[reg_y as usize],
                    Param::Val(val_y)   => val_y,
                };
                self.registers[x as usize] *= factor;
            }
            Instruction::Mod(x, y) => {
                let modulo = match y {
                    Param::Reg(reg_y) => self.registers[reg_y as usize],
                    Param::Val(val_y)   => val_y,
                };
                self.registers[x as usize] %= modulo;
            }
            Instruction::Rcv(x) => {
                if !self.part2_rcv {
                    // part 1 behaviour
                    if self.registers[x as usize] != 0 {
                        self.rcv_queue.push_back(self.snd_output.pop_back().unwrap());
                        return Err(ExecBreakReason::RcvWait);
                    }
                } else {
                    // part 2 behaviour
                    if self.rcv_queue.is_empty() {
                        // reset PC and signal that we are waiting for rcv data
                        self.pc -= 1;
                        return Err(ExecBreakReason::RcvWait)
                    } else {
                        self.registers[x as usize] = self.rcv_queue.pop_front().unwrap();
                    }
                }
            }
            Instruction::Jgz(x, y) => {
                let cmp_val: Word = match x {
                    Param::Reg(reg_x) => self.registers[reg_x as usize],
                    Param::Val(val_x)   => val_x
                };
                let offset: Word = match y {
                    Param::Reg(reg_y) => self.registers[reg_y as usize],
                    Param::Val(val_y)   => val_y
                };
                if cmp_val > 0 {
                    self.pc -= 1;  // reset previous inc
                    self.pc += offset as i8;
                }
            }
        }
        if self.pc < 0 || self.pc >= self.instructions.len() as i8 {
            Err(ExecBreakReason::EndOfProgram)  // does not happen with our input
        } else {
            Ok(())
        }
    }
    fn execute(&mut self) -> ExecBreakReason {
        loop {
            if let Err(err) = self.execute_one_instruction() {
                return err;
            }
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let mut computer = Computer::new(input_text, false);
    computer.execute();
    computer.rcv_queue.pop_front().unwrap().to_string()
}

fn solve_part2(input_text: String) -> String {
    // setup
    let mut comp0 = Computer::new(input_text.clone(), true);
    comp0.registers[Register::P as usize] = 0;
    let mut comp1 = Computer::new(input_text, true);
    comp1.registers[Register::P as usize] = 1;

    loop {
        // let both run
        comp0.execute();
        comp1.execute();

        // data exchange
        comp0.snd_output.drain(0..)
            .for_each(|item|
                comp1.rcv_queue.push_back(item)
            );
        comp1.snd_output.drain(0..)
            .for_each(|item|
                comp0.rcv_queue.push_back(item)
            );

        // termination condition
        if comp0.rcv_queue.is_empty() && comp1.rcv_queue.is_empty() {
            break;
        }
    }
    comp1.snd_counter.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 18)
            .expect("Could not fetch input");

    println!("Example: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "4");
    }
}
