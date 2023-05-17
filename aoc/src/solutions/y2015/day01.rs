#[crate::aoc(year = 2015, day = 1, part = "A")]
fn day01a(chars: &[u8]) -> Option<i64> {
    day01(chars)
        .last()
        .map(|state| state.0)
}

#[crate::aoc(year = 2015, day = 1, part = "B")]
fn day01b(chars: &[u8]) -> Option<usize> {
    day01(chars)
        .find(|&state| state.0 < 0)
        .map(|state| state.1)
}

fn day01(chars: &[u8]) -> impl Iterator<Item = (i64, usize)> + '_ {
    chars
        .iter()
        .scan((0, 0), |state, &c| {
            state.1 += 1;
            state.0 += match c {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            };

            Some(state.clone())
        })
}

#[cfg(test)]
mod tests_y2015_day01 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day01a(b"(())"), Some(0));
        assert_eq!(day01a(b"()()"), Some(0));
        assert_eq!(day01a(b"((("), Some(3));
        assert_eq!(day01a(b"(()(()("), Some(3));
        assert_eq!(day01a(b"))((((("), Some(3));
        assert_eq!(day01a(b"())"), Some(-1));
        assert_eq!(day01a(b"))("), Some(-1));
        assert_eq!(day01a(b")))"), Some(-3));
        assert_eq!(day01a(b")())())"), Some(-3));
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day01b(b")"), Some(1));
        assert_eq!(day01b(b"()())"), Some(5));
    }
}
