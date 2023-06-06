use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum State {
    Flying,
    Resting,
}

impl Default for State {
    fn default() -> Self {
        Self::Flying
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Reindeer {
    name: String,
    distance: u64,
    score: u64,
    speed: u64,
    flying_timer: usize,
    rest_timer: usize,
    timer: usize,
    state: State,
}

impl Reindeer {
    fn get_distance(&self) -> u64 {
        self.distance
    }

    fn get_score(&self) -> u64 {
        self.score
    }

    fn update(&mut self) {
        match self.state {
            State::Flying => self.distance += self.speed,
            State::Resting => {},
        }

        self.update_timer();
    }

    fn update_timer(&mut self) {
        self.timer -= 1;
        if self.timer == 0 {
            (self.state, self.timer) = match self.state {
                State::Flying => (State::Resting, self.rest_timer),
                State::Resting => (State::Flying, self.flying_timer),
            }
        }
    }
}

impl FromStr for Reindeer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        let name = parts[0].into();
        let speed = parts[3].parse()?;
        let flying_timer = parts[6].parse()?;
        let rest_timer = parts[13].parse()?;

        Ok(Reindeer {
            name,
            speed,
            flying_timer,
            rest_timer,
            timer: flying_timer,
            ..Default::default()
        })
    }
}

fn reindeer_games<F>(lines: &[&str], seconds: usize, selector: F) -> u64
where F: Fn(&Reindeer) -> u64
{
    let reindeers: Result<Vec<Reindeer>, _> = lines
        .into_iter()
        .map(|&line| line.parse())
        .collect();
    if reindeers.is_err() {
        return 0;
    }

    let mut reindeers = reindeers.unwrap();

    for _ in 0..seconds {
        reindeers.iter_mut()
            .for_each(|reindeer| reindeer.update());
        let max_distance = reindeers.iter()
            .map(|r| r.distance)
            .max()
            .unwrap();
        reindeers.iter_mut().for_each(|reindeer| {
            if reindeer.distance == max_distance {
                reindeer.score += 1;
            }
        })
    }

    reindeers
        .iter()
        .map(|r| selector(r))
        .max()
        .unwrap()
}

fn day14(lines: &[&str], seconds: usize, is_part_b: bool) -> u64 {
    reindeer_games(
        lines,
        seconds,
        if !is_part_b { Reindeer::get_distance } else { Reindeer::get_score },
    )
}

#[crate::aoc(year = 2015, day = 14, part = "A")]
fn day14a(lines: &[&str]) -> u64 {
    day14(lines, 2503, false)
}

#[crate::aoc(year = 2015, day = 14, part = "B")]
fn day14b(lines: &[&str]) -> u64 {
    day14(lines, 2503, true)
}

#[cfg(test)]
mod tests_y2015_day14 {
    use super::*;

    const INPUT: &[&str] = &[
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
    ];

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day14(INPUT, 1000, false), 1120);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day14(INPUT, 1000, true), 689);
    }
}
