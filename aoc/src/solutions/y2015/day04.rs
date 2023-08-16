use crate::common::*;

fn count_leading_zeroes(digest: &Digest) -> usize {
    let mut count = 0;
    let bytes = **digest;

    for i in 0..16 {
        let byte = bytes[i];
        if byte == 0x00 {
            count += 2;
        } else if byte < 0x10 {
            count += 1;
            break;
        } else {
            break;
        }
    }

    count
}

fn day04(input: &str, num_leading_zeroes: usize) -> u64 {
    let mut original = State::new();
    original.update(input);
    let mut n = 1;

    loop {
        let mut state = original.clone();
        state.update(n.to_string());
        let digest = state.digest();

        if count_leading_zeroes(&digest) >= num_leading_zeroes {
            break;
        }

        n += 1;
    }

    n
}

#[crate::aoc(year = 2015, day = 4, part = "A")]
fn day04a(input: &str) -> u64 {
    day04(input.trim_end(), 5)
}

#[crate::aoc(year = 2015, day = 4, part = "B")]
fn day04b(input: &str) -> u64 {
    day04(input.trim_end(), 6)
}

#[cfg(test)]
mod tests_y2015_day04 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day04a("abcdef"), 609043);
        assert_eq!(day04a("pqrstuv"), 1048970);
    }
}
