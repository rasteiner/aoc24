use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Instruction (OpCode, u8);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OpCode {
    ADv = 0, // regA / comboOp^2 -> regA
    Bxl = 1, // bitwise xor of regB and literal -> regB
    Bst = 2, // combo % 8 -> regB
    Jnz = 3, // if regA != 0 { jump to literal (don't increase pc after jump) }
    Bxc = 4, // bitwise xor of regB and regC -> regB
    Out = 5, // combo % 8 -> output
    BDv = 6, // regA / comboOp^2 -> regB
    CDv = 7, // regA / comboOp^2 -> regC
}

impl TryFrom<i64> for OpCode {
    type Error = ();

    fn try_from(s: i64) -> Result<Self, Self::Error> {
        match s {
            0 => Ok(OpCode::ADv),
            1 => Ok(OpCode::Bxl),
            2 => Ok(OpCode::Bst),
            3 => Ok(OpCode::Jnz),
            4 => Ok(OpCode::Bxc),
            5 => Ok(OpCode::Out),
            6 => Ok(OpCode::BDv),
            7 => Ok(OpCode::CDv),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Program {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    pc: usize,
    instructions: Vec<Instruction>,
    output: Vec<i64>,
    code: Vec<i64>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let reg_a = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let reg_b = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let reg_c = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let code = lines.skip(1).next().unwrap().split_whitespace().last().unwrap().split(',').map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        let instructions = code.iter()
            .collect::<Vec<&i64>>()
            .chunks(2)
            .map(|c| Instruction((*c[0]).try_into().unwrap(), (*c[1]).try_into().unwrap()))
            .collect();

        Ok(Program {
            reg_a,
            reg_b,
            reg_c,
            pc: 0,
            code,
            instructions,
            output: Vec::new(),
        })
    }
}

impl Program {
    
    fn get_combo_value(&self, combo: u8) -> i64 {
        match combo {
            0..4 => combo as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo value"),
        }
    }

    fn run(&mut self) -> usize {
        self.output.clear();

        let mut counter = 0;
        
        while self.pc < self.instructions.len() {
            let Instruction(op, arg) = self.instructions[self.pc];
            counter += 1;

            match op {
                OpCode::ADv => self.reg_a /= 2i64.pow(self.get_combo_value(arg) as u32),
                OpCode::Bxl => self.reg_b ^= arg as i64,
                OpCode::Bst => self.reg_b = self.get_combo_value(arg) % 8,
                OpCode::Jnz => if self.reg_a != 0 { self.pc = arg as usize; continue; },
                OpCode::Bxc => self.reg_b ^= self.reg_c,
                OpCode::Out => self.output.push(self.get_combo_value(arg) % 8),
                OpCode::BDv => self.reg_b = self.reg_a / 2i64.pow(self.get_combo_value(arg) as u32),
                OpCode::CDv => self.reg_c = self.reg_a / 2i64.pow(self.get_combo_value(arg) as u32),
            }
            
            self.pc += 1;
        }

        counter
    }

    fn get_combo_desc(&self, arg: u8) -> String {
        match arg {
            0 => String::from("0"),
            1 => String::from("1"),
            2 => String::from("2"),
            3 => String::from("3"),
            4 => String::from("reg_a"),
            5 => String::from("reg_b"),
            6 => String::from("reg_c"),
            _ => unreachable!()
        }
    }

    /**
     * Transpile the program to a string in a single expression.
     */
    fn transpile(&self) -> String {
        self.instructions.iter().map(|Instruction(op, arg)| {
            match op {
                OpCode::ADv | OpCode::BDv | OpCode::CDv => {
                    let reg = match op {
                        OpCode::ADv => "a",
                        OpCode::BDv => "b",
                        OpCode::CDv => "c",
                        _ => unreachable!()
                    };
                    match arg {
                        0 => format!("reg_{} = reg_a;", reg),
                        1..4 => format!("reg_{} = reg_a >> {};", reg, arg),
                        _ => format!("reg_{} = reg_a >> {};", reg, self.get_combo_desc(*arg)),
                    }
                },
                
                OpCode::Bxl => format!("reg_b ^= {};", arg),
                OpCode::Bst => format!("reg_b = {} % 8;", self.get_combo_desc(*arg)),
                OpCode::Jnz => format!("if reg_a != 0 {{ GOTO {} }};", arg),
                OpCode::Bxc => "reg_b ^= reg_c;".to_string(),
                OpCode::Out => format!("output.push({} % 8);", self.get_combo_desc(*arg)),
                
            }
        }).collect::<Vec<String>>().join("\n")
    }

    fn run_for(&mut self, output: &Vec<i64>, id: i64) {
        self.output.clear();
        
        let mut next_to_match = output.iter().peekable();

        while self.pc < self.instructions.len() {
            let Instruction(op, arg) = self.instructions[self.pc];

            match op {
                OpCode::ADv => self.reg_a /= 2i64.pow(self.get_combo_value(arg) as u32),
                OpCode::Bxl => self.reg_b ^= arg as i64,
                OpCode::Bst => self.reg_b = self.get_combo_value(arg) % 8,
                OpCode::Jnz => if self.reg_a != 0 { self.pc = arg as usize; continue; },
                OpCode::Bxc => self.reg_b ^= self.reg_c,
                OpCode::Out => {
                    let o = self.get_combo_value(arg) % 8;
                    if let Some(&n) = next_to_match.peek() {
                        if o != *n {
                            return;
                        }
                        self.output.push(o);
                        next_to_match.next();
                    } else {
                        return;
                    }
                },
                OpCode::BDv => self.reg_b = self.reg_a / 2i64.pow(self.get_combo_value(arg) as u32),
                OpCode::CDv => self.reg_c = self.reg_a / 2i64.pow(self.get_combo_value(arg) as u32),
            }
            
            self.pc += 1;
        }
    }
}


pub fn part1(input: &String) -> Box<dyn ToString> {
    let mut program: Program = input.parse().unwrap();
    let steps = program.run();
    println!("{:?} steps", steps);

    Box::new(program.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
}

pub fn part2(input: &String) -> Box<dyn ToString> {

    let mut program: Program = input.parse().unwrap();
    let code = program.code.clone();
    let mut run = |n: i64| {
        program.reg_a = n;
        program.pc = 0;
        program.output.clear();
        program.run();
        program.output[0]
    };

    let mut find_for = |base: i64, n: i64| {
        (0..=100).find(|a| {
            run(base | *a) == n
        }).unwrap()
    };

    let mut n = 0;
    for c in code.iter().rev() {
        n = n << 3 | find_for(n << 3, *c);
    }

    run(n);

    println!("Program OUT: {:?}", program.output);

    Box::new(n)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT_1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0"
    };
    const TEST_RESULT_1: &str = "4,6,3,5,6,3,5,2,1,0";

    const TEST_INPUT_2: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0"
    };
    const TEST_RESULT_2: i64 = 117440;

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT_1)).to_string(), TEST_RESULT_1.to_string());
    }

    // Test for part2
    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT_2)).to_string(), TEST_RESULT_2.to_string());
    }
    
}