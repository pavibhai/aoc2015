use crate::day23::Instruction::{HLF, INC, JIE, JIO, JMP, TPL};

pub fn generator(input: &str) -> Program {
    Program::parse(input).unwrap()
}

pub fn part1(p: &Program) -> u32 {
    let mut c = Computer::new();
    c.run(p);
    c.reg_b
}

pub fn part2(p: &Program) -> u32 {
    let mut c = Computer::new();
    c.reg_a = 1;
    c.run(p);
    c.reg_b
}

pub struct Program {
    inst: Vec<Instruction>,
}

impl Program {
    fn parse(input: &str) -> Result<Self, String> {
        let inst = input
            .lines()
            .map(Instruction::parse)
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Program { inst })
    }
}

#[derive(Debug)]
struct Computer {
    reg_a: u32,
    reg_b: u32,
    addr: u32,
}

impl Computer {
    fn new() -> Self {
        Computer {
            reg_a: 0,
            reg_b: 0,
            addr: 0,
        }
    }

    fn run(&mut self, program: &Program) {
        while self.addr < program.inst.len() as u32 {
            let i = &program.inst[self.addr as usize];
            self.addr += 1;
            //println!("I: {:?}", i);
            self._process_inst(i);
            //println!("C: {:?}", self);
        }
    }

    fn _process_inst(&mut self, inst: &Instruction) {
        match inst {
            HLF(Register::A) => self.reg_a /= 2,
            HLF(Register::B) => self.reg_b /= 2,
            TPL(Register::A) => self.reg_a *= 3,
            TPL(Register::B) => self.reg_b *= 3,
            INC(Register::A) => self.reg_a += 1,
            INC(Register::B) => self.reg_b += 1,
            JMP(delta) => self.addr = (self.addr as i32 + *delta - 1) as u32,
            JIE(Register::A, o) if self.reg_a % 2 == 0 => {
                self.addr = (self.addr as i32 + *o - 1) as u32
            }
            JIE(Register::B, o) if self.reg_b % 2 == 0 => {
                self.addr = (self.addr as i32 + *o - 1) as u32
            }
            JIO(Register::A, o) if self.reg_a == 1 => {
                self.addr = (self.addr as i32 + *o - 1) as u32
            }
            JIO(Register::B, o) if self.reg_b == 1 => {
                self.addr = (self.addr as i32 + *o - 1) as u32
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

impl Register {
    fn parse(s: &str) -> Result<Register, String> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => panic!("invalid register"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(i32),
    JIE(Register, i32),
    JIO(Register, i32),
}

impl Instruction {
    fn parse(input: &str) -> Result<Instruction, String> {
        let mut parts = input.split(" ");
        let i = parts.next();
        let o1 = parts.next().map(|r| r.trim_suffix(','));
        let o2 = parts.next();
        let result = match i {
            Some("hlf") => HLF(Register::parse(o1.ok_or("invalid register")?)?),
            Some("tpl") => TPL(Register::parse(o1.ok_or("invalid register")?)?),
            Some("inc") => INC(Register::parse(o1.ok_or("invalid register")?)?),
            Some("jmp") => JMP(o1
                .ok_or("invalid register")?
                .parse()
                .map_err(|e| format!("{}", e))?),
            Some("jie") => JIE(
                Register::parse(o1.ok_or("invalid register")?)?,
                o2.ok_or("invalid instruction")?
                    .parse()
                    .map_err(|e| format!("{}", e))?,
            ),
            Some("jio") => JIO(
                Register::parse(o1.ok_or("invalid register")?)?,
                o2.ok_or("invalid instruction")?
                    .parse()
                    .map_err(|e| format!("{}", e))?,
            ),
            _ => panic!("invalid instruction"),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "inc b
jio b, +2
tpl b
inc b";

    #[test]
    fn test_generator() {
        let p = generator(INPUT);
        assert_eq!(4, p.inst.len());
    }

    #[test]
    fn test_part1() {
        let p = generator(INPUT);
        assert_eq!(2, part1(&p));
    }

    #[test]
    fn test_part2() {
        let p = generator(INPUT);
        assert_eq!(2, part2(&p));
    }
}
