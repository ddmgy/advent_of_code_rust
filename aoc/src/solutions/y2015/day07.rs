use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Error<'a> {
    WireNumberParse(&'a str),
    UnknownOperator(&'a str),
    InvalidNumberOfArguments(usize),
    InstructionParse(&'a str),
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::WireNumberParse(s) => write!(f, "unable to parse u16 from `{}`", s)?,
            Self::UnknownOperator(s) => write!(f, "unknown operator: `{}`", s)?,
            Self::InvalidNumberOfArguments(n) => write!(f, "invalid number of arguments: {}", n)?,
            Self::InstructionParse(s) => write!(f, "unable to parse Instruction from `{}`", s)?,
        }

        Ok(())
    }
}

impl<'a> std::error::Error for Error<'a> {}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Wire<'a> {
    Name(&'a str),
    Number(u16),
}

impl<'a> Wire<'a> {
    fn parse(s: &'a str) -> Result<Self, Error<'a>> {
        match s.chars().nth(0) {
            None => Err(Error::WireNumberParse(s)),
            Some(c) if c.is_ascii_digit() => {
                match u16::from_str_radix(s, 10) {
                    Ok(num) => Ok(Self::Number(num)),
                    Err(_) => Err(Error::WireNumberParse(s)),
                }
            },
            Some(_) => Ok(Self::Name(s)),
        }
    }
}

impl<'a> From<&'a str> for Wire<'a> {
    fn from(value: &'a str) -> Self {
        Self::Name(value)
    }
}

impl<'a> From<&'a Wire<'a>> for Wire<'a> {
    fn from(value: &'a Wire<'a>) -> Self {
        *value
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Gate<'a> {
    Set(Wire<'a>),
    Not(Wire<'a>),
    And(Wire<'a>, Wire<'a>),
    Or(Wire<'a>, Wire<'a>),
    LShift(Wire<'a>, Wire<'a>),
    RShift(Wire<'a>, Wire<'a>),
}

impl<'a> Gate<'a> {
    fn parse(parts: &[&'a str]) -> Result<Self, Error<'a>> {
        match parts.len() {
            1 => Ok(Self::Set(Wire::parse(parts[0])?)),
            2 => Ok(Self::Not(Wire::parse(parts[1])?)),
            3 => {
                let left = Wire::parse(parts[0])?;
                let right = Wire::parse(parts[2])?;
                match parts[1] {
                    "AND" => Ok(Self::And(left, right)),
                    "OR" => Ok(Self::Or(left, right)),
                    "LSHIFT" => Ok(Self::LShift(left, right)),
                    "RSHIFT" => Ok(Self::RShift(left, right)),
                    _ => Err(Error::UnknownOperator(parts[1])),
                }
            },
            n => Err(Error::InvalidNumberOfArguments(n)),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Instruction<'a> {
    input: Gate<'a>,
    output: &'a str,
}

impl<'a> Instruction<'a> {
    fn new(line: &'a str) -> Result<Self, Error<'a>> {
        let parts: Vec<&str> = line.split(" ").collect();
        if let Some(index) = parts.iter().position(|&p| p == "->") {
            let (left, right) = parts.split_at(index);

            Gate::parse(left).map(|input| {
                Self {
                    input,
                    output: right[1],
                }
            })
        } else {
            Err(Error::InstructionParse(line))
        }
    }
}

struct Circuit<'a> {
    instructions: Vec<Instruction<'a>>,
    values: HashMap<&'a str, u16>,
}

impl<'a> Circuit<'a> {
    fn new(lines: &'a [&str]) -> Result<Self, Error<'a>> {
        let res: Result<Vec<_>, _> = lines
            .into_iter()
            .map(|&line| Instruction::new(line))
            .collect();

        res.map(|instructions| {
            let mut instructions = instructions;
            let mut outstructions = vec![]; // lol
            let mut end = instructions.len();
            let mut known: HashSet<&str> = HashSet::new();

            while !instructions.is_empty() {
                'inner: for i in 0..end {
                    let all_inputs_known = match &instructions[i].input {
                        Gate::Set(wire) => match wire {
                            Wire::Number(_) => true,
                            Wire::Name(name) => known.contains(name),
                        },
                        Gate::Not(wire) => match wire {
                            Wire::Number(_) => true,
                            Wire::Name(name) => known.contains(name),
                        },
                        Gate::And(left, right) => match (left, right) {
                            (Wire::Number(_), Wire::Number(_)) => true,
                            (Wire::Name(n1), Wire::Name(n2)) => known.contains(n1) && known.contains(n2),
                            (Wire::Name(name), _) => known.contains(name),
                            (_, Wire::Name(name)) => known.contains(name),
                        },
                        Gate::Or(left, right) => match (left, right) {
                            (Wire::Number(_), Wire::Number(_)) => true,
                            (Wire::Name(n1), Wire::Name(n2)) => known.contains(n1) && known.contains(n2),
                            (Wire::Name(name), _) => known.contains(name),
                            (_, Wire::Name(name)) => known.contains(name),
                        },
                        Gate::LShift(left, right) => match (left, right) {
                            (Wire::Number(_), Wire::Number(_)) => true,
                            (Wire::Name(n1), Wire::Name(n2)) => known.contains(n1) && known.contains(n2),
                            (Wire::Name(name), _) => known.contains(name),
                            (_, Wire::Name(name)) => known.contains(name),
                        },
                        Gate::RShift(left, right) => match (left, right) {
                            (Wire::Number(_), Wire::Number(_)) => true,
                            (Wire::Name(n1), Wire::Name(n2)) => known.contains(n1) && known.contains(n2),
                            (Wire::Name(name), _) => known.contains(name),
                            (_, Wire::Name(name)) => known.contains(name),
                        },
                    };

                    if all_inputs_known {
                        let inst = instructions.swap_remove(i);
                        known.insert(inst.output);
                        outstructions.push(inst);
                        end -= 1;
                        break 'inner;
                    }
                }
            }

            Self {
                instructions: outstructions,
                values: HashMap::new(),
            }
        })
    }

    fn run(&mut self) -> Result<(), ()> {
        for instruction in self.instructions.iter() {
            let value = match &instruction.input {
                Gate::Set(wire) => match self.get(wire) {
                    Some(n) => n,
                    None => return Err(()),
                },
                Gate::Not(wire) => match self.get(wire) {
                    Some(n) => !n,
                    None => return Err(()),
                },
                Gate::And(left, right) => match (self.get(left), self.get(right)) {
                    (Some(a), Some(b)) => a & b,
                    (_, _) => return Err(()),
                },
                Gate::Or(left, right) => match (self.get(left), self.get(right)) {
                    (Some(a), Some(b)) => a | b,
                    (_, _) => return Err(()),
                },
                Gate::LShift(left, right) => match (self.get(left), self.get(right)) {
                    (Some(a), Some(b)) => a << b,
                    (_, _) => return Err(()),
                },
                Gate::RShift(left, right) => match (self.get(left), self.get(right)) {
                    (Some(a), Some(b)) => a >> b,
                    (_, _) => return Err(()),
                },
            };

            self.values.insert(instruction.output, value);
        }

        Ok(())
    }

    fn override_wire(&mut self, wire: &'a str, b: u16) {
        let inst = Instruction {
            input: Gate::Set(Wire::Number(b)),
            output: wire,
        };
        if let Some(index) = self.instructions.iter().position(|i| i.output == wire) {
            self.instructions[index] = inst;
        }
    }

    fn get<W>(&self, wire: W) -> Option<u16>
    where W: Into<Wire<'a>>
    {
        match wire.into() {
            Wire::Number(n) => Some(n),
            Wire::Name(name) => match self.values.get(name) {
                Some(&value) => Some(value),
                None => None,
            }
        }
    }
}

fn day07(lines: &[&str], times: usize) -> Option<u16> {
    match Circuit::new(lines) {
        Ok(mut circuit) => {
            let mut a: Option<u16> = None;

            for _ in 0..times {
                a = match circuit.run() {
                    Ok(_) => circuit.get("a"),
                    Err(_) => return None,
                };

                circuit.override_wire("b", a.unwrap());
            }

            a
        },
        Err(e) => {
            eprintln!("{e}");
            None
        },
    }
}

#[crate::aoc(year = 2015, day = 7, part = "A")]
fn day07a(lines: &[&str]) -> Option<u16> {
    day07(lines, 1)
}

#[crate::aoc(year = 2015, day = 7, part = "B")]
fn day07b(lines: &[&str]) -> Option<u16> {
    day07(lines, 2)
}

#[cfg(test)]
mod tests_y2015_day07 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
    }
}
