use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Cell(i64, i64);

impl Cell {
    fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    #[inline(always)]
    fn neighbors(&self) -> Vec<Cell> {
        let &Self(x, y) = self;

        vec![
            Self(x - 1, y - 1),
            Self(x, y - 1),
            Self(x + 1, y - 1),
            Self(x - 1, y),
            Self(x + 1, y),
            Self(x - 1, y + 1),
            Self(x, y + 1),
            Self(x + 1, y + 1),
        ]
    }
}

struct GameOfLife {
    width: usize,
    height: usize,
    cells: HashSet<Cell>,
}

impl GameOfLife {
    fn cell_in_range(&self, cell: &Cell) -> bool {
        let width = self.width as i64;
        let height = self.height as i64;

        cell.0 >= 0 && cell.0 < width && cell.1 >= 0 && cell.1 < height
    }

    fn stick_on_corners(&mut self) {
        let x = (self.width as i64) - 1;
        let y = (self.height as i64) - 1;
        self.cells.insert(Cell::new(0, 0));
        self.cells.insert(Cell::new(x, 0));
        self.cells.insert(Cell::new(0, y));
        self.cells.insert(Cell::new(x, y));
    }

    fn neighbor_counts(&self) -> HashMap<Cell, usize> {
        let mut counts = HashMap::new();

        for cell in self.cells.iter()
            .flat_map(Cell::neighbors)
            .filter(|cell| self.cell_in_range(cell))
        {
            *counts.entry(cell).or_insert(0) += 1;
        }

        counts
    }

    fn next_generation(&mut self) {
        let mut gen = self.neighbor_counts()
            .into_iter()
            .filter_map(|(cell, count)| {
                match (self.cells.contains(&cell), count) {
                    (true, 2) | (_, 3) => Some(cell.clone()),
                    _ => None,
                }
            })
            .collect();

        std::mem::swap(&mut self.cells, &mut gen);
    }

    fn living_cells(&self) -> usize {
        self.cells.len()
    }
}

impl std::fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = Cell::new(x as i64, y as i64);
                if self.cells.contains(&cell) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::str::FromStr for GameOfLife {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut cells = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            let line_width = line.len();

            for (x, c) in line.bytes().enumerate() {
                if c == b'#' {
                    cells.insert(Cell::new(x as i64, y as i64));
                }
            }

            width = std::cmp::max(width, line_width);
            height += 1;
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}

fn day18(input: &str, steps: usize, is_part_b: bool) -> usize {
    match input.parse::<GameOfLife>() {
        Ok(mut life) => {
            if is_part_b {
                life.stick_on_corners();
            }

            for _ in 0..steps {
                life.next_generation();

                if is_part_b {
                    life.stick_on_corners();
                }
            }

            life.living_cells()
        },
        Err(_) => 0,
    }
}

#[crate::aoc(year = 2015, day = 18, part = "A")]
fn day18a(input: &str) -> usize {
    day18(input, 100, false)
}

#[crate::aoc(year = 2015, day = 18, part = "B")]
fn day18b(input: &str) -> usize {
    day18(input, 100, true)
}

#[cfg(test)]
mod tests_y2015_day18 {
    use super::*;

    const INPUT: &str = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day18(INPUT, 4, false), 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day18(INPUT, 5, true), 17);
    }
}
