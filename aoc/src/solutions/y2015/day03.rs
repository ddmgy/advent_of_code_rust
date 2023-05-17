use std::collections::HashSet;
use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Self::new(0, 0)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<u8> for Point {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Point::new(0, 1),
            b'<' => Point::new(-1, 0),
            b'>' => Point::new(1, 0),
            b'v' => Point::new(0, -1),
            _ => panic!("unable to create Point from u8")
        }
    }
}

fn simulate_santas(input: &str, count: usize) -> usize {
    let mut santas = vec![Point::origin(); count];
    let (mut i, mut ni) = (0, count - 1);
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point::origin());

    let chars = input.trim().as_bytes();
    for &c in chars.iter() {
        let dir = Point::from(c);
        santas[i] += dir;
        visited.insert(santas[i].clone());
        (i, ni) = (ni, i);
    }

    visited.len()
}

#[crate::aoc(year = 2015, day = 3, part = "A")]
fn day03a(input: &str) -> usize {
    simulate_santas(input, 1)
}

#[crate::aoc(year = 2015, day = 3, part = "B")]
fn day03b(input: &str) -> usize {
    simulate_santas(input, 2)
}

#[cfg(test)]
mod tests_y2015_day03{
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day03a(">"), 2);
        assert_eq!(day03a("^>v<"), 4);
        assert_eq!(day03a("^v^v^v^v^v"), 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day03b("^v"), 3);
        assert_eq!(day03b("^>v<"), 3);
        assert_eq!(day03b("^v^v^v^v^v"), 11);
    }
}
