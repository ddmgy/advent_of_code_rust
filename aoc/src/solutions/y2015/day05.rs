use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
enum FailureReason<'a> {
    Rule1(usize),
    Rule2,
    Rule3(&'a str),
    Rule4,
    Rule5,
}

impl<'a> std::error::Error for FailureReason<'a> {}

impl<'a> fmt::Display for FailureReason<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Rule1(num_vowels) => write!(f, "expected at least three vowels, got {}", num_vowels)?,
            Self::Rule2 => write!(f, "expected at least one letter that appears twice in a row")?,
            Self::Rule3(disallowed) => write!(f, "contains disallowed string: `{}`", disallowed)?,
            Self::Rule4 => write!(f, "expected pair of any two letters that appear at least twice without overlap")?,
            Self::Rule5 => write!(f, "expected at least one letter which repeats with exactly one letter between them")?,
        };

        Ok(())
    }
}

fn rule_1(password: &[u8]) -> Result<(), FailureReason> {
    let mut count = 0;
    for &b in password.iter() {
        match b {
            b'a' | b'e' | b'i' | b'o' | b'u' => count += 1,
            _ => {},
        }
    }

    if count >= 3 {
        Ok(())
    } else {
        Err(FailureReason::Rule1(count))
    }
}

fn rule_2(password: &[u8]) -> Result<(), FailureReason> {
    for i in 0..password.len() - 1 {
        if password[i] == password[i + 1] {
            return Ok(());
        }
    }

    Err(FailureReason::Rule2)
}

fn rule_3(password: &[u8]) -> Result<(), FailureReason> {
    for i in 0..password.len() - 1 {
        match (password[i], password[i + 1]) {
            (b'a', b'b') => return Err(FailureReason::Rule3("ab")),
            (b'c', b'd') => return Err(FailureReason::Rule3("cd")),
            (b'p', b'q') => return Err(FailureReason::Rule3("pq")),
            (b'x', b'y') => return Err(FailureReason::Rule3("xy")),
            _ => {}
        }
    }

    Ok(())
}

fn rule_4(password: &[u8]) -> Result<(), FailureReason> {
    let len = password.len();
    for i in 0..len - 3 {
        for j in i + 2..len - 1 {
            if password[i] == password[j] && password[i + 1] == password[j + 1] {
                return Ok(());
            }
        }
    }

    Err(FailureReason::Rule4)
}

fn rule_5(password: &[u8]) -> Result<(), FailureReason> {
    for i in 0..password.len() - 2 {
        if password[i] == password[i + 2] {
            return Ok(());
        }
    }

    Err(FailureReason::Rule5)
}

fn naughty_or_nice<F>(password: &str, rules: Vec<F>) -> Result<(), FailureReason>
where F: Fn(&[u8]) -> Result<(), FailureReason>
{
    let bytes = password.as_bytes();

    for rule in rules {
        let _ = rule(bytes)?;
    }

    Ok(())
}

fn naughty_or_nice_v1(password: &str) -> Result<(), FailureReason> {
    naughty_or_nice(password, vec![
        rule_1,
        rule_2,
        rule_3,
    ])
}

fn naughty_or_nice_v2(password: &str) -> Result<(), FailureReason> {
    naughty_or_nice(password, vec![
        rule_4,
        rule_5,
    ])
}

fn day05<F>(passwords: &[&str], f: F) -> usize
where F: Fn(&str) -> Result<(), FailureReason>
{
    let nice = passwords
        .iter()
        .filter(|password| f(password).is_ok())
        .collect::<Vec<_>>();

    nice.len()
}

#[crate::aoc(year = 2015, day = 5, part = "A")]
fn day05a(passwords: &[&str]) -> usize {
    day05(passwords, naughty_or_nice_v1)
}

#[crate::aoc(year = 2015, day = 5, part = "B")]
fn day05b(passwords: &[&str]) -> usize {
    day05(passwords, naughty_or_nice_v2)
}

#[cfg(test)]
mod tests_y2015_day05{
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day05a(&["ugknbfddgicrmopn"]), 1);
        assert_eq!(rule_1(b"ugknbfddgicrmopn"), Ok(()));
        assert_eq!(rule_2(b"ugknbfddgicrmopn"), Ok(()));
        assert_eq!(rule_3(b"ugknbfddgicrmopn"), Ok(()));

        assert_eq!(day05a(&["aaa"]), 1);
        assert_eq!(rule_1(b"aaa"), Ok(()));
        assert_eq!(rule_2(b"aaa"), Ok(()));
        assert_eq!(rule_3(b"aaa"), Ok(()));

        assert_eq!(day05a(&["jchzalrnumimnmhp"]), 0);
        assert_eq!(rule_1(b"jchzalrnumimnmhp"), Ok(()));
        assert_eq!(rule_2(b"jchzalrnumimnmhp"), Err(FailureReason::Rule2));
        assert_eq!(rule_3(b"jchzalrnumimnmhp"), Ok(()));

        assert_eq!(day05a(&["haegwjzuvuyypxyu"]), 0);
        assert_eq!(rule_1(b"haegwjzuvuyypxyu"), Ok(()));
        assert_eq!(rule_2(b"haegwjzuvuyypxyu"), Ok(()));
        assert_eq!(rule_3(b"haegwjzuvuyypxyu"), Err(FailureReason::Rule3("xy")));

        assert_eq!(day05a(&["dvszwmarrgswjxmb"]), 0);
        assert_eq!(rule_1(b"dvszwmarrgswjxmb"), Err(FailureReason::Rule1(1)));
        assert_eq!(rule_2(b"dvszwmarrgswjxmb"), Ok(()));
        assert_eq!(rule_3(b"dvszwmarrgswjxmb"), Ok(()));
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day05b(&["qjhvhtzxzqqjkmpb"]), 1);
        assert_eq!(rule_4(b"qjhvhtzxzqqjkmpb"), Ok(()));
        assert_eq!(rule_5(b"qjhvhtzxzqqjkmpb"), Ok(()));

        assert_eq!(day05b(&["xxyxx"]), 1);
        assert_eq!(rule_4(b"xxyxx"), Ok(()));
        assert_eq!(rule_5(b"xxyxx"), Ok(()));

        assert_eq!(day05b(&["uurcxstgmygtbstg"]), 0);
        assert_eq!(rule_4(b"uurcxstgmygtbstg"), Ok(()));
        assert_eq!(rule_5(b"uurcxstgmygtbstg"), Err(FailureReason::Rule5));

        assert_eq!(day05b(&["ieodomkazucvgmuy"]), 0);
        assert_eq!(rule_4(b"ieodomkazucvgmuy"), Err(FailureReason::Rule4));
        assert_eq!(rule_5(b"ieodomkazucvgmuy"), Ok(()));
    }
}
