/// Find the first house to which the number of presents is >= `target`.
/// Each elf can deliver `mul` presents to up to `limit` houses.
fn day20(target: usize, mul: usize, limit: usize) -> Option<usize> {
    let d = target / mul;
    let mut counts = vec![0usize; d];
    for i in 1..=d {
        for j in 1..=std::cmp::min(d / i, limit) {
            counts[j * i - 1] += i * mul;
        }

        if counts[i - 1] >= target {
            return Some(i);
        }
    }

    None
}

#[crate::aoc(year = 2015, day = 20, part = "A")]
fn day20a(input: &str) -> Option<usize> {
    day20(input.trim().parse().unwrap(), 10, usize::MAX)
}

#[crate::aoc(year = 2015, day = 20, part = "B")]
fn day20b(input: &str) -> Option<usize> {
    day20(input.trim().parse().unwrap(), 11, 50)
}

#[cfg(test)]
mod tests_y2015_day20 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day20a("10"), Some(1));
        assert_eq!(day20a("20"), Some(2));
        assert_eq!(day20a("30"), Some(2));
        assert_eq!(day20a("40"), Some(3));
        assert_eq!(day20a("60"), Some(4));
        assert_eq!(day20a("100"), Some(6));
        assert_eq!(day20a("150"), Some(8));
    }
}
