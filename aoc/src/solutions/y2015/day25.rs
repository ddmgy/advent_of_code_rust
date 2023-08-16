use regex::{Captures, Regex};

struct Code(u64);

impl Code {
    fn new(code: u64) -> Self {
        Self(code)
    }
}

impl Iterator for Code {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.0;
        self.0 = (curr * 252533) % 33554393;

        Some(curr)
    }
}

fn parse_row_and_column(input: &str) -> Option<(u64, u64)> {
    let re = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();
    let caps = re.captures(input)?;
    let parse_int = |caps: &Captures<'_>, idx| caps.get(idx).unwrap().as_str().parse().unwrap();

    let target_row = parse_int(&caps, 1);
    let target_column = parse_int(&caps, 2);

    Some((target_row, target_column))
}

#[crate::aoc(year = 2015, day = 25, part = "A")]
fn day25a(input: &str) -> Option<u64> {
    let (target_row, target_column) = parse_row_and_column(input)?;
    let mut code = Code::new(20151125).into_iter();
    let mut row = 1;
    let mut column = 1;
    let mut next_row = row + 1;

    loop {
        if row == target_row && column == target_column {
            break;
        }

        if row == 1 {
            row = next_row;
            next_row += 1;
            column = 1;
        } else {
            row -= 1;
            column += 1;
        }

        code.next();
    }

    Some(code.0)
}

#[crate::aoc(year = 2015, day = 25, part = "B")]
fn day25b(input: &str) -> Option<usize> {
    todo!();
}

#[cfg(test)]
mod tests_y2015_day25 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        let input = "To continue, please consult the code grid in the manual.  Enter the code at row 5, column 6.";
        assert_eq!(day25a(input), Some(31_663_883));
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
    }
}
