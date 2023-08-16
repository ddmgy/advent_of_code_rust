use std::collections::HashSet;

#[derive(Debug)]
struct PasswordGenerator {
    curr: Vec<u8>,
}

impl PasswordGenerator {
    fn new(curr: &[u8]) -> Self {
        Self {
            curr: curr.to_vec(),
        }
    }

    fn passes_requirement_1(&self) -> bool {
        self.curr.windows(3)
            .map(|window| (
                window[1].saturating_sub(window[0]),
                window[2].saturating_sub(window[1]),
            ))
            .any(|steps| steps == (1, 1))
    }

    fn passes_requirement_2(&self) -> bool {
        for c in &self.curr {
            match c {
                b'i' | b'l' | b'o' => return false,
                _ => {},
            }
        }

        true
    }

    fn passes_requirement_3(&self) -> bool {
        let mut seen = HashSet::new();
        let mut i = 0;

        while i < 7 {
            let c = self.curr[i];
            if c == self.curr[i + 1] {
                seen.insert(c);
                i += 1;
            }

            i += 1;
        }

        seen.len() > 1
    }

    fn advance(&mut self) {
        let mut i = 7isize;
        while i >= 0 {
            let c = self.curr.get_mut(i as usize).unwrap();
            match *c {
                b'z' => *c = b'a',
                _ => {
                    *c += 1;
                    break;
                },
            }

            i -= 1;
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance();

        while !self.passes_requirement_1()
            || !self.passes_requirement_2()
            || !self.passes_requirement_3()
        {
            self.advance();
        }

        Some(String::from_utf8(self.curr.clone()).unwrap())
    }
}

fn day11(input: &[u8], nth: usize) -> String {
    PasswordGenerator::new(&input[0..8]).nth(nth).unwrap()
}

#[crate::aoc(year = 2015, day = 11, part = "A")]
fn day11a(input: &[u8]) -> String {
    day11(input, 0)
}

#[crate::aoc(year = 2015, day = 11, part = "B")]
fn day11b(input: &[u8]) -> String {
    day11(input, 1)
}

#[cfg(test)]
mod tests_y2015_day11 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        let gen = PasswordGenerator::new(b"hijklmmn");
        assert!(gen.passes_requirement_1());
        assert!(!gen.passes_requirement_2());
        assert!(!gen.passes_requirement_3());

        let gen = PasswordGenerator::new(b"abbceffg");
        assert!(!gen.passes_requirement_1());
        assert!(gen.passes_requirement_2());
        assert!(gen.passes_requirement_3());

        let gen = PasswordGenerator::new(b"abbcegjk");
        assert!(!gen.passes_requirement_1());
        assert!(gen.passes_requirement_2());
        assert!(!gen.passes_requirement_3());

        let mut gen = PasswordGenerator::new(b"abcdefgh");
        assert_eq!(gen.next(), Some(String::from("abcdffaa")));

        let mut gen = PasswordGenerator::new(b"ghijklmn");
        assert_eq!(gen.next(), Some(String::from("ghjaabcc")));
    }
}
