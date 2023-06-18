use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    Replacement(String),
    Input,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Replacement(s) => write!(f, "unable to parse replacement: '{}'", s)?,
            Self::Input => write!(f, "unable to parse input")?,
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
struct Replacement(String, String);

impl std::str::FromStr for Replacement {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.split_once(" => ") {
            None => Err(Error::Replacement(value.into())),
            Some((from, to)) => Ok(Self(from.into(), to.into())),
        }
    }
}

struct Input {
    replacements: Vec<Replacement>,
    original: String,
}

impl std::str::FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut replacements = vec![];
        let mut original: Option<String> = None;

        for line in s.trim().lines() {
            match line {
                line if line.contains(" => ") => replacements.push(line.parse()?),
                line if line.is_empty() => {},
                line => original = Some(line.into()),
            }
        }

        match original {
            None => Err(Error::Input),
            Some(original) => Ok(Self {
                replacements,
                original,
            })
        }
    }
}

#[crate::aoc(year = 2015, day = 19, part = "A")]
fn day19a(input: &str) -> usize {
    match input.parse() {
        Ok(Input { replacements, original }) => {
            let mut molecules: HashSet<String> = HashSet::new();

            for Replacement(from, to) in replacements {
                for (i, _) in original.match_indices(&from) {
                    let mut molecule = String::new();
                    molecule.push_str(&original[..i]);
                    molecule.push_str(&to);
                    molecule.push_str(&original[i + from.len()..]);
                    molecules.insert(molecule);
                }
            }

            molecules.len()
        },
        Err(e) => {
            eprintln!("error: {}", e);
            0
        },
    }
}

#[crate::aoc(year = 2015, day = 19, part = "B", version = "do_the_work")]
fn day19b_do_the_work(input: &str) -> usize {
    // For posterity: This works for the example inputs, but stalls on
    // the actual input. I suspect I could use a queue to repeatedly test all
    // substitutions from original => 'e', but that would likely take an
    // exceedingly long time, and the below method has worked for the past 8 years, so...

    match input.parse() {
        Ok(Input { mut replacements, mut original }) => {
            replacements.sort_unstable_by(|a, b| b.1.len().partial_cmp(&a.1.len()).unwrap());
            let mut steps = 0;
            let end = String::from("e");

            while original != end {
                for Replacement(from, to) in &replacements {
                    let mut replacement_found: Option<String> = None;

                    for (i, _) in original.match_indices(to) {
                        let mut molecule = String::new();
                        molecule.push_str(&original[..i]);
                        molecule.push_str(&from);
                        molecule.push_str(&original[i + to.len()..]);

                        replacement_found = Some(molecule);
                        break;
                    }

                    if let Some(replacement) = replacement_found {
                        original = replacement;
                        steps += 1;
                        break;
                    }
                }
            }

            steps
        },
        Err(e) => {
            eprintln!("error: {}", e);
            0
        },
    }
}

#[crate::aoc(year = 2015, day = 19, part = "B")]
fn day19b(input: &str) -> usize {
    let molecule = input.trim().lines().last().unwrap();
    let num_symbols = molecule.chars().filter(|c| c.is_uppercase()).count();
    let count_rn = molecule.match_indices("Rn").count();
    let count_ar = molecule.match_indices("Ar").count();
    let count_y = molecule.match_indices("Y").count();

    num_symbols - count_rn - count_ar - 2 * count_y - 1
}

#[cfg(test)]
mod tests_y2015_day19 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day19a("H => HO\nH => OH\nO => HH\n\n\nHOH"), 4);
        assert_eq!(day19a("H => HO\nH => OH\nO => HH\n\n\nHOHOHO"), 7);
        assert_eq!(day19a("H => 00\n\n\nH2O"), 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day19b_do_the_work("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH"), 3);
        assert_eq!(day19b_do_the_work("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO"), 6);
    }
}
