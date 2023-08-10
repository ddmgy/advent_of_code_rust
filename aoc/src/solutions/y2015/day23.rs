#[derive(Debug)]
enum Error {
    ParseInstruction,
    ParseRegister,
    ParseOffset(std::num::ParseIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseInstruction => write!(f, "unable to parse instruction")?,
            Self::ParseRegister => write!(f, "unable to parse register")?,
            Self::ParseOffset(e) => write!(f, "unable to parse offset: {:?}", e)?,
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
enum Register { A, B }

impl std::str::FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(Error::ParseRegister),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(|c| c == ' ' || c == ',').collect::<Vec<_>>();

        match parts[0] {
            "hlf" => {
                let register = parts[1].parse()?;

                Ok(Self::Hlf(register))
            },
            "tpl" => {
                let register = parts[1].parse()?;

                Ok(Self::Tpl(register))
            },
            "inc" => {
                let register = parts[1].parse()?;

                Ok(Self::Inc(register))
            },
            "jmp" => {
                let offset = parts[1].parse()
                    .map_err(|e| Error::ParseOffset(e))?;

                Ok(Self::Jmp(offset))
            },
            "jie" => {
                let register = parts[1].parse()?;
                let offset = parts[3].parse()
                    .map_err(|e| Error::ParseOffset(e))?;

                Ok(Self::Jie(register, offset))
            },
            "jio" => {
                let register = parts[1].parse()?;
                let offset = parts[3].parse()
                    .map_err(|e| Error::ParseOffset(e))?;

                Ok(Self::Jio(register, offset))
            },
            _ => Err(Error::ParseInstruction),
        }
    }
}

fn run_program(instructions: Vec<Instruction>, is_part_b: bool) -> Option<u64> {
    use Instruction::*;

    let len = instructions.len() as isize;
    let mut ip = 0isize;
    let mut a = if is_part_b { 1 } else { 0 };
    let mut b = 0;

    macro_rules! reg {
        ($r:expr) => {
            match $r {
                Register::A => &mut a,
                Register::B => &mut b,
            }
        };
    }

    while ip >= 0 && ip < len {
        let inst = &instructions[ip as usize];

        match inst {
            Hlf(r) => {
                let reg = reg!(r);
                *reg /= 2;
                ip += 1;
            },
            Tpl(r) => {
                let reg = reg!(r);
                *reg *= 3;
                ip += 1;
            },
            Inc(r) => {
                let reg = reg!(r);
                *reg += 1;
                ip += 1;
            },
            Jmp(offset) => {
                ip += *offset as isize;
            },
            Jie(r, offset) => {
                let reg = reg!(r);
                ip += if *reg % 2 == 0 {
                    *offset as isize
                } else {
                        1
                };
            },
            Jio(r, offset) => {
                let reg = reg!(r);
                ip += if *reg == 1 {
                    *offset as isize
                } else {
                    1
                }
            },
        }
    }

    Some(b)
}

fn day23(input: &[&str], is_part_b: bool) -> Option<u64> {
    let instructions: Result<Vec<Instruction>, _> = input.iter()
        .map(|line| line.parse())
        .collect();

    match instructions {
        Ok(instructions) => run_program(instructions, is_part_b),
        Err(e) => {
            eprintln!("error: {:}", e);
            None
        },
    }
}

#[crate::aoc(year = 2015, day = 23, part = "A")]
fn day23a(input: &[&str]) -> Option<u64> {
    day23(input, false)
}

#[crate::aoc(year = 2015, day = 23, part = "B")]
fn day23b(input: &[&str]) -> Option<u64> {
    day23(input, true)
}

#[cfg(test)]
mod tests_y2015_day23 {
    use super::*;

    // Change the registers from `a` to `b` so I don't have to
    // write any logic for grabbing a specific register.
    const TEST_PROGRAM: &[&str] = &[
        "inc b",
        "jio b, +2",
        "tpl b",
        "inc b",
    ];

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day23a(TEST_PROGRAM), Some(2));
    }
}
