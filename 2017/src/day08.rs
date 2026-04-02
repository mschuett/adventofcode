use std::collections::HashMap;
use crate::helper;

enum Condition {
    Ge, Gt, Le, Lt, Eq, Ne,
}
impl Condition {
    fn eval(&self, a: i32, b: i32) -> bool {
        match &self {
            Condition::Ge => a >= b,
            Condition::Gt => a > b,
            Condition::Le => a <= b,
            Condition::Lt => a < b,
            Condition::Eq => a == b,
            Condition::Ne => a != b,
        }
    }
}
impl TryFrom<&str> for Condition {
    type Error = String;
    fn try_from(s: &str) -> Result<Condition, Self::Error> {
        match s {
            ">=" => Ok(Condition::Ge),
            ">" => Ok(Condition::Gt),
            "<=" => Ok(Condition::Le),
            "<" => Ok(Condition::Lt),
            "==" => Ok(Condition::Eq),
            "!=" => Ok(Condition::Ne),
            _ => Err(format!("unknown condition {}", s)),
        }
    }
}
enum Op {
    Inc, Dec
}
impl TryFrom<&str> for Op {
    type Error = String;
    fn try_from(s: &str) -> Result<Op, Self::Error> {
        match s {
            "inc" => Ok(Op::Inc),
            "dec" => Ok(Op::Dec),
            _ => Err(format!("unknown op {}", s)),
        }
    }
}
impl Op {
    fn eval(&self, a: i32, b: i32) -> i32 {
        match self {
            Op::Inc => a + b,
            Op::Dec => a - b,
        }
    }
}
struct Instruction{
    register: String,
    operation: Op,
    operation_value: i32,
    condition: Condition,
    condition_register: String,
    condition_value: i32,
}
impl TryFrom<&str> for Instruction {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Instruction, Self::Error> {
        let words = s.trim().split_whitespace().collect::<Vec<_>>();
        assert_eq!(words.len(), 7);
        let register = words[0].to_string();
        let operation = Op::try_from(words[1]).unwrap();
        let operation_value = words[2].parse().unwrap();
        assert_eq!(words[3], "if");
        let condition_register = words[4].to_string();
        let condition = Condition::try_from(words[5]).unwrap();
        let condition_value = words[6].parse().unwrap();
        Ok(Instruction{ register, operation, operation_value, condition, condition_register, condition_value })
    }
}
impl Instruction {
    fn exec(&self, registers: &mut HashMap<String, i32>) {
        let cond_reg_value = registers
            .entry(self.condition_register.clone())
            .or_insert(0);
        if self.condition.eval(*cond_reg_value, self.condition_value) {
            let reg_value = *registers.get(&self.register).unwrap_or(&0);
            let new_value = self.operation.eval(reg_value, self.operation_value);
            registers.insert(self.register.clone(), new_value);
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let program: Vec<Instruction> = input_text
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<_, _>>()
        .unwrap();
    let mut registers: HashMap<String, i32> = HashMap::new();

    for instruction in program {
        instruction.exec(&mut registers);
    }
    // println!("registers: {:?}", registers);
    let (key, value) = registers.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
    println!("largest value in register {}: {}", key, value);
    value.to_string()
}

fn solve_part2(input_text: String) -> String {
    let program: Vec<Instruction> = input_text
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<_, _>>()
        .unwrap();
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max_reg_value: i32 = 0;

    for instruction in program {
        instruction.exec(&mut registers);
        let current_max = *registers.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap().1;
        if current_max > max_reg_value {
            max_reg_value = current_max;
        }
    }
    max_reg_value.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 8)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "1");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "10");
    }
}
