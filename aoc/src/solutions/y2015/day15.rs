#[derive(Debug)]
enum Error {
    Input(String),
    ParseNumber(std::num::ParseIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(s) => write!(f, "unable to parse ingredient from '{}'", s)?,
            Self::ParseNumber(e) => write!(f, "unable to parse property: {}", e)?,
        };

        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseNumber(value)
    }
}

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl std::str::FromStr for Ingredient {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .split(" ")
            .map(|s| s.trim_end_matches(|c| c == ',' || c == ':'))
            .collect();

        if parts.len() != 11 {
            Err(Error::Input(s.into()))
        } else {
            let capacity = parts[2].parse()?;
            let durability = parts[4].parse()?;
            let flavor = parts[6].parse()?;
            let texture = parts[8].parse()?;
            let calories = parts[10].parse()?;

            Ok(Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            })
        }
    }
}

fn get_portions(max: usize, amount: usize) -> Vec<Vec<usize>> {
    if amount == 1 {
        vec![vec![max]]
    } else {
        (0..=max)
            .flat_map(|x| {
                let mut result = get_portions(max - x, amount - 1);
                for comb in result.iter_mut() {
                    comb.push(x);
                }

                result
            })
            .collect()
    }
}

fn day15(lines: &[&str], is_part_2: bool) -> i64 {
    let ingredients: Result<Vec<Ingredient>, _> = lines
        .into_iter()
        .map(|line| line.parse())
        .collect();

    match ingredients {
        Ok(ingredients) => {
            let mut max_score = i64::MIN;

            'outer: for portions in get_portions(100, ingredients.len()) {
                macro_rules! property_score {
                    (__impl $property:ident) => ({
                        ingredients
                            .iter()
                            .enumerate()
                            .map(|(i, ingredient)| ingredient.$property * (portions[i] as i64))
                            .sum::<i64>()
                    });

                    ($property:ident) => ({
                        property_score!(__impl $property)
                    });

                    ($property:ident, $($properties:ident),+) => ({
                        let score = property_score!(__impl $property);
                        if score <= 0 {
                            continue 'outer;
                        }

                        score * property_score!($($properties),+)
                    });
                }

                if is_part_2 && property_score!(calories) != 500 {
                    continue 'outer;
                }

                let total_score = property_score!(capacity, durability, flavor, texture);
                max_score = std::cmp::max(max_score, total_score);
            }

            max_score
        },
        Err(e) => {
            eprintln!("error: {}", e);
            0
        },
    }
}

#[crate::aoc(year = 2015, day = 15, part = "A")]
fn day15a(lines: &[&str]) -> i64 {
    day15(lines, false)
}

#[crate::aoc(year = 2015, day = 15, part = "B")]
fn day15b(lines: &[&str]) -> i64 {
    day15(lines, true)
}

#[cfg(test)]
mod tests_y2015_day15 {
    use super::*;

    const INPUT: &[&str] = &[
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
        "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
    ];

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day15a(INPUT), 62_842_880);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day15b(INPUT), 57_600_000);
    }
}
