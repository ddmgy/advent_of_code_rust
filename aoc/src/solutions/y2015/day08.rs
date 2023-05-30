#[inline]
fn characters_in_code(s: &[u8]) -> usize {
    s.len()
}

fn characters_in_memory(s: &[u8]) -> usize {
    let (mut start, end) = (1, s.len() - 1);
    let mut count = 0;

    while start < end {
        count += 1;
        if s[start] == b'\\' {
            if s[start + 1] == b'x' {
                start += 3;
            } else {
                start += 1;
            }
        }

        start += 1;
    }

    count
}

fn encoded_length(s: &[u8]) -> usize {
    let mut count = 2;

    for &c in s.iter() {
        count += match c {
            b'"' | b'\\' => 2,
            _ => 1,
        };
    }

    count
}

fn day08<F>(lines: &[&[u8]], f: F) -> usize
where F: Fn(&[u8]) -> (usize, usize)
{
    let (left, right) = lines
        .into_iter()
        .map(|line| f(line))
        .fold(
            (0, 0),
            |acc, el| (acc.0 + el.0, acc.1 + el.1),
        );

    left - right
}

#[crate::aoc(year = 2015, day = 8, part = "A")]
fn day08a(lines: &[&[u8]]) -> usize {
    day08(lines, |line| (
        characters_in_code(line),
        characters_in_memory(line),
    ))
}

#[crate::aoc(year = 2015, day = 8, part = "B")]
fn day08b(lines: &[&[u8]]) -> usize {
    day08(lines, |line| (
        encoded_length(line),
        characters_in_code(line),
    ))
}

#[cfg(test)]
mod tests_y2015_day08 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(characters_in_code(br#""""#), 2);
        assert_eq!(characters_in_memory(br#""""#), 0);

        assert_eq!(characters_in_code(br#""abc""#), 5);
        assert_eq!(characters_in_memory(br#""abc""#), 3);

        assert_eq!(characters_in_code(br#""aaa\"aaa""#), 10);
        assert_eq!(characters_in_memory(br#""aaa\"aaa""#), 7);

        assert_eq!(characters_in_code(br#""\x27""#), 6);
        assert_eq!(characters_in_memory(br#""\x27""#), 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(encoded_length(br#""""#), 6);
        assert_eq!(encoded_length(br#""abc""#), 9);
        assert_eq!(encoded_length(br#""aaa\"aaa""#), 16);
        assert_eq!(encoded_length(br#""\x27""#), 11);
    }
}
