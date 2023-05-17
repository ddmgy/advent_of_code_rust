use nom::{
    bytes::streaming::tag,
    character::complete::digit1,
    sequence::tuple, IResult, combinator::map_res,
};

use crate::error::Error;

#[derive(Debug)]
struct Present {
    l: u64,
    w: u64,
    h: u64,
}

impl Present {
    fn new(input: &str) -> Result<Self, Error> {
        let mut parser = tuple((
            dimension,
            tag("x"),
            dimension,
            tag("x"),
            dimension,
        ));

        match parser(input) {
            Ok((_, (a, _, b, _, c))) => {
                let mut parts = [a, b, c];
                parts.sort();

                Ok(Present {
                    l: parts[0],
                    w: parts[1],
                    h: parts[2],
                })
            },
            Err(e) => {
                eprintln!("{:?}", e);

                Err(Error::Parse(format!("unable to parse Present from '{}'", input)))
            }
        }
    }

    fn wrapping_paper_needed(&self) -> u64 {
        let Self { l, w, h } = self;
        3 * l * w + 2 * w * h + 2 * h * l
    }

    fn ribbon_needed(&self) -> u64 {
        let Self { l, w, h } = self;
        2 * l + 2 * w + l * w * h
    }
}

fn dimension_parse(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 10)
}

fn dimension(input: &str) -> IResult<&str, u64> {
    map_res(
        digit1,
        dimension_parse,
    )(input)
}

#[crate::aoc(year = 2015, day = 2, part = "A")]
fn day02a(lines: Vec<&str>) -> Option<u64> {
    let presents: Result<Vec<_>, Error> = lines
        .into_iter()
        .map(Present::new)
        .collect();


    match presents {
        Ok(presents) => Some(presents
                .into_iter()
                .map(|p| p.wrapping_paper_needed())
                .sum()
        ),
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}

#[crate::aoc(year = 2015, day = 2, part = "B")]
fn day02b(lines: Vec<&str>) -> Option<u64> {
    let presents: Result<Vec<_>, Error> = lines
        .into_iter()
        .map(Present::new)
        .collect();

    match presents {
        Ok(presents) => Some(presents
            .into_iter()
            .map(|p| p.ribbon_needed())
            .sum()
        ),
        Err(e) => {
            eprintln!("{:?}", e);
            None
        },
    }
}

#[cfg(test)]
mod tests_y2015_day02{
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day02a(vec!["2x3x4"]), Some(58));
        assert_eq!(day02a(vec!["1x1x10"]), Some(43));
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day02b(vec!["2x3x4"]), Some(34));
        assert_eq!(day02b(vec!["1x1x10"]), Some(14));
    }
}
