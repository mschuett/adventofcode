use crate::helper;

// re-using from day 18
type Word = i64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Register {
    A = 0, B, C, D, E, F, G, H
}
impl TryFrom<&str> for Register {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        assert_eq!(s.len(), 1);
        let c = s.chars().next().unwrap();
        match c {
            'a' => Ok(Register::A),
            'b' => Ok(Register::B),
            'c' => Ok(Register::C),
            'd' => Ok(Register::D),
            'e' => Ok(Register::E),
            'f' => Ok(Register::F),
            'g' => Ok(Register::G),
            'h' => Ok(Register::H),
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
    Set(Register, Param),
    Sub(Register, Param),
    Mul(Register, Param),
    Jnz(Param, Param),
}
impl TryFrom<&str> for Instruction {
    type Error = String;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let words = line.split_whitespace().collect::<Vec<_>>();

        match words[0] {
            "set" => Ok(Self::Set(Register::try_from(words[1])?, Param::try_from(words[2])?)),
            "sub" => Ok(Self::Sub(Register::try_from(words[1])?, Param::try_from(words[2])?)),
            "mul" => Ok(Self::Mul(Register::try_from(words[1])?, Param::try_from(words[2])?)),
            "jnz" => Ok(Self::Jnz(Param::try_from(words[1])?, Param::try_from(words[2])?)),
            _ => Err(format!("Unknown instruction {}", line)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExecBreakReason {
    EndOfProgram
}

#[derive(Clone, Debug)]
struct Computer {
    registers: [Word; 8],  // with nightly API: [Word; mem::variant_count::<Register>()]
    pc: i8,
    instructions: Vec<Instruction>,
    mult_counter: u32,
}
impl Computer {
    fn new(input_text: String) -> Computer {
        let instructions: Vec<Instruction> = input_text.lines()
            .map(Instruction::try_from)
            .collect::<Result<_, _>>()
            .unwrap();
        Computer {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            pc: 0,
            instructions,
            mult_counter: 0,
        }
    }
    fn execute_one_instruction(&mut self) -> Result<(), ExecBreakReason> {
        let instruction = self.instructions[self.pc as usize];
        self.pc += 1;
        match instruction {
            Instruction::Set(x, y) => {
                let value = match y {
                    Param::Reg(reg_y) => self.registers[reg_y as usize],
                    Param::Val(val_y) => val_y,
                };
                self.registers[x as usize] = value;
            }
            Instruction::Sub(x, y) => {
                match y {
                    Param::Reg(reg_y) => self.registers[x as usize] -= self.registers[reg_y as usize],
                    Param::Val(val_y)   => self.registers[x as usize] -= val_y
                }
            }
            Instruction::Mul(x, y) => {
                let factor = match y {
                    Param::Reg(reg_y) => self.registers[reg_y as usize],
                    Param::Val(val_y)   => val_y,
                };
                self.registers[x as usize] *= factor;
                self.mult_counter += 1;
            }
            Instruction::Jnz(x, y) => {
                let cmp_val: Word = match x {
                    Param::Reg(reg_x) => self.registers[reg_x as usize],
                    Param::Val(val_x)   => val_x
                };
                let offset: Word = match y {
                    Param::Reg(reg_y) => self.registers[reg_y as usize],
                    Param::Val(val_y)   => val_y
                };
                if cmp_val != 0 {
                    self.pc -= 1;  // reset previous inc
                    self.pc += offset as i8;
                }
            }
        }
        if self.pc < 0 || self.pc >= self.instructions.len() as i8 {
            Err(ExecBreakReason::EndOfProgram)
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
    let mut computer = Computer::new(input_text);
    computer.execute();
    computer.mult_counter.to_string()
}

fn solve_part2(input_text: String) -> String {
    let computer = Computer::new(input_text);
    let Instruction::Set(_, Param::Val(init_value)) = computer.instructions[0] else { panic!() };

    // did a manual disassembly
    // this is a re-implementation to not use the interpreter
    fn is_prime(n: u32) -> bool {
        for i in 2..=n.isqrt() {
            if n.is_multiple_of(i) {
                return false;
            }
        }
        true
    }
    let mut composite_counter = 0u32;
    let b = init_value as u32 * 100 + 100000;
    let c = b + 17000;
    for n in (b..=c).step_by(17) {
        if !is_prime(n) {
            composite_counter += 1;
        }
    }
    composite_counter.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 23)
        .expect("Could not fetch input");

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}
