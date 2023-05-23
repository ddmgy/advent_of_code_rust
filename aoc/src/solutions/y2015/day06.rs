use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Error {
    InstructionKindParse,
    RangeParse,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InstructionKindParse => write!(f, "unable to parse instruction kind"),
            Self::RangeParse => write!(f, "unable to parse range"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug)]
enum InstructionKind {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Clone, Debug)]
struct Instruction {
    kind: InstructionKind,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn instruction_kind(input: &str) -> IResult<&str, InstructionKind> {
    map_res(
        alt((
            tag("turn on "),
            tag("turn off "),
            tag("toggle "),
        )),
        |s| match s {
            "turn on " => Ok(InstructionKind::TurnOn),
            "turn off " => Ok(InstructionKind::TurnOff),
            "toggle " => Ok(InstructionKind::Toggle),
            _ => Err(Error::InstructionKindParse),
        },
    )(input)
}

fn range(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, x) = map_res(
        take_while1(|c: char| c.is_digit(10)),
        |s: &str| s.parse::<usize>(),
    )(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = map_res(
        take_while1(|c: char| c.is_digit(10)),
        |s: &str| s.parse::<usize>(),
    )(input)?;

    Ok((input, (x, y)))
}

impl Instruction {
    fn new(input: &str) -> IResult<&str, Self> {
        match tuple((instruction_kind, range, tag(" through "), range))(input) {
            Ok((input, (kind, r1, _, r2))) => {
                let (x1, y1) = r1;
                let (x2, y2) = r2;

                Ok((
                    input,
                    Self {
                        kind,
                        x1,
                        y1,
                        x2,
                        y2,
                    }
                ))
            },
            Err(e) => Err(e),
        }
    }
}

trait Grid {
    fn interpret(&mut self, instruction: Instruction);

    fn total_brightness(self) -> usize;
}

struct BoolGrid {
    data: Vec<bool>,
}

impl BoolGrid {
    fn new() -> Self {
        Self {
            data: vec![false; 1000 * 1000],
        }
    }
}

impl Grid for BoolGrid {
    fn interpret(&mut self, instruction: Instruction) {
        let Instruction { kind, x1, y1, x2, y2 } = instruction;

        for y in y1..=y2 {
            for x in x1..=x2 {
                let i = y * 1000 + x;
                if let Some(b) = self.data.get_mut(i) {
                    match kind {
                        InstructionKind::TurnOn => *b = true,
                        InstructionKind::TurnOff => *b = false,
                        InstructionKind::Toggle => *b = !*b,
                    }
                }
            }
        }
    }

    fn total_brightness(self) -> usize {
        self.data
            .into_iter()
            .filter(|&b| b)
            .count()
    }
}

struct IntGrid {
    data: Vec<usize>,
}

impl IntGrid {
    fn new() -> Self {
        Self {
            data: vec![0; 1000 * 1000],
        }
    }
}

impl Grid for IntGrid {
    fn interpret(&mut self, instruction: Instruction) {
        let Instruction { kind, x1, y1, x2, y2 } = instruction;

        for y in y1..=y2 {
            for x in x1..=x2 {
                let i = y * 1000 + x;
                if let Some(n) = self.data.get_mut(i) {
                    match kind {
                        InstructionKind::TurnOn => *n = n.saturating_add(1),
                        InstructionKind::TurnOff => *n = n.saturating_sub(1),
                        InstructionKind::Toggle => *n = n.saturating_add(2),
                    }
                }
            }
        }
    }

    fn total_brightness(self) -> usize {
        self.data
            .into_iter()
            .sum()
    }
}

fn day06<F, G>(lines: &[&str], make_grid: F) -> usize
where
    F: Fn() -> G,
    G: Grid,
{
    let mut grid = make_grid();
    for &line in lines.iter() {
        match Instruction::new(line) {
            Ok((_, instruction)) => {
                grid.interpret(instruction);
            },
            Err(e) => eprintln!("{e}"),
        }
    }

    grid.total_brightness()
}

#[crate::aoc(year = 2015, day = 6, part = "A")]
fn day06a(lines: &[&str]) -> usize {
    day06(lines, BoolGrid::new)
}

#[crate::aoc(year = 2015, day = 6, part = "B")]
fn day06b(lines: &[&str]) -> usize {
    day06(lines, IntGrid::new)
}

#[cfg(test)]
mod tests_y2015_day06{
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        let res = day06a(&[
            "turn on 0,0 through 999,999",
            "toggle 0,0 through 999,0",
            "turn off 499,499 through 500,500",
        ]);
        assert_eq!(res, 1000 * 1000 - 1000 - 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        let res = day06b(&[
            "turn on 0,0 through 0,0",
            "toggle 0,0 through 999,999",
        ]);
        assert_eq!(res, 1 + 2 * 1000 * 1000);
    }
}
