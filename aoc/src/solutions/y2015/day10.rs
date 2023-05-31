struct LookAndSay {
    curr: Vec<u8>,
    prev: Vec<u8>,
}

impl LookAndSay {
    fn new(data: &str) -> Self {
        Self {
            curr: data.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
            prev: Vec::new(),
        }
    }
}

impl Iterator for LookAndSay {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        std::mem::swap(&mut self.prev, &mut self.curr);
        self.curr.clear();

        let mut count = 1;
        let mut curr = self.prev[0];

        for &n in &self.prev[1..] {
            if curr == n {
                count += 1;
            } else {
                self.curr.push(count);
                self.curr.push(curr);
                count = 1;
                curr = n;
            }
        }

        self.curr.push(count);
        self.curr.push(curr);

        Some(self.prev.clone())
    }
}

fn look_and_say(s: &str, n: usize) -> usize {
    match LookAndSay::new(s).nth(n) {
        Some(seq) => seq.len(),
        None => 0,
    }
}

#[inline]
fn day10(input: &str, n: usize) -> usize {
    look_and_say(input.trim_end(), n)
}

#[crate::aoc(year = 2015, day = 10, part = "A")]
fn day10a(input: &str) -> usize {
    day10(input, 40)
}

#[crate::aoc(year = 2015, day = 10, part = "B")]
fn day10b(input: &str) -> usize {
    day10(input, 50)
}

#[cfg(test)]
mod tests_y2015_day10 {
    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        let mut it = super::LookAndSay::new("211");
        assert_eq!(it.next(), Some(vec![2, 1, 1]));

        let mut it = super::LookAndSay::new("1");
        assert_eq!(it.next(), Some(vec![1]));
        assert_eq!(it.next(), Some(vec![1, 1]));
        assert_eq!(it.next(), Some(vec![2, 1]));
        assert_eq!(it.next(), Some(vec![1, 2, 1, 1]));
        assert_eq!(it.next(), Some(vec![1, 1, 1, 2, 2, 1]));
        assert_eq!(it.next(), Some(vec![3, 1, 2, 2, 1, 1]));
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
    }
}
