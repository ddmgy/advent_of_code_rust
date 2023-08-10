#[derive(Debug)]
enum Error {
    ParseNumber(std::num::ParseIntError),
    UnknownProperty(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseNumber(e) => write!(f, "error parsing number: {}", e)?,
            Self::UnknownProperty(s) => write!(f, "unknown property: '{}'", s)?,
        }

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
struct Sue {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl std::str::FromStr for Sue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(" ")
            .map(|part| part.trim_end_matches(|c| c == ':' || c == ','))
            .collect::<Vec<_>>();

        let mut number = None;
        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;

        for i in 0..parts.len() / 2 {
            match (parts[i * 2], parts[i * 2 + 1].parse::<usize>()?) {
                ("Sue", n) => number = Some(n),
                ("children", n) => children = Some(n),
                ("cats", n) => cats = Some(n),
                ("samoyeds", n) => samoyeds = Some(n),
                ("pomeranians", n) => pomeranians = Some(n),
                ("akitas", n) => akitas = Some(n),
                ("vizslas", n) => vizslas = Some(n),
                ("goldfish", n) => goldfish = Some(n),
                ("trees", n) => trees = Some(n),
                ("cars", n) => cars = Some(n),
                ("perfumes", count) => perfumes = Some(count),
                (property, _) => return Err(Error::UnknownProperty(property.into())),
            }
        }

        Ok(Self {
            number: number.unwrap(),
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        })
    }
}

fn mfcsam_v1(known: &Sue, unknown: &Sue) -> bool {
    macro_rules! unroll {
        (__impl $known:expr, $unknown:expr; $property:ident) => {
            if $unknown.$property.is_some() {
                $known.$property == $unknown.$property
            } else {
                true
            }
        };

        ($known:expr, $unknown:expr; $property:ident) => {
            unroll!(__impl $known, $unknown; $property)
        };

        ($known:expr, $unknown:expr; $property:ident, $($properties:ident),+ $(,)?) => {
            unroll!(__impl $known, $unknown; $property)
                && unroll!($known, $unknown; $($properties),+)
        };
    }

    unroll!(
        known, unknown;
        children,
        cats,
        samoyeds,
        pomeranians,
        akitas,
        vizslas,
        goldfish,
        trees,
        cars,
        perfumes,
    )
}

fn mfcsam_v2(known: &Sue, unknown: &Sue) -> bool {
    macro_rules! unroll {
        (__impl $known:expr, $unknown:expr; $property:ident, $cmp:tt) => {
            if $unknown.$property.is_some() {
                let a = $known.$property.unwrap();
                let b = $unknown.$property.unwrap();
                a $cmp b
            } else {
                true
            }
        };

        ($known: expr, $unknown:expr; $property:ident, $cmp:tt) => {
            unroll!(__impl $known, $unknown; $property, $cmp)
        };

        ($known:expr, $unknown:expr; $property:ident, $cmp:tt, $($properties:ident, $cmps:tt),+ $(,)?) => {
            unroll!(__impl $known, $unknown; $property, $cmp)
                && unroll!($known, $unknown; $($properties, $cmps),+)
        };
    }

    unroll!(
        known, unknown;
        children, ==,
        cats, <,
        samoyeds, ==,
        pomeranians, >,
        akitas, ==,
        vizslas, ==,
        goldfish, >,
        trees, <,
        cars, ==,
        perfumes, ==,
    )
    // false
}

fn day16<F>(lines: &[&str], mfcsam: F) -> Option<usize>
where F: Fn(&Sue, &Sue) -> bool
{
    let sues: Result<Vec<Sue>, _> = lines
        .into_iter()
        .map(|line| line.parse())
        .collect();

    let known = Sue {
        number: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    match sues {
        Ok(sues) => {
            sues
                .iter()
                .find(|unknown| mfcsam(&known, unknown))
                .map(|sue| sue.number)
        },
        Err(e) => {
            eprintln!("error: {:?}", e);
            None
        },
    }
}

#[crate::aoc(year = 2015, day = 16, part = "A")]
fn day16a(lines: &[&str]) -> Option<usize> {
    day16(lines, mfcsam_v1)
}

#[crate::aoc(year = 2015, day = 16, part = "B")]
fn day16b(lines: &[&str]) -> Option<usize> {
    day16(lines, mfcsam_v2)
}
