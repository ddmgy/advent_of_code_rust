use std::collections::{HashMap, HashSet};

use crate::common::PermutationsExt;

fn day09<F>(lines: &[&str], f: F) -> usize
where F: Fn(usize, usize) -> usize
{
    let mut edges: HashMap<(&str, &str), usize> = HashMap::new();
    let mut nodes: HashSet<&str> = HashSet::new();

    for line in lines {
        let parts: Vec<_> = line.split(" ").collect();
        let (a, b, distance) = (parts[0], parts[2], parts[4].parse().unwrap());
        nodes.insert(a);
        nodes.insert(b);
        edges.insert((a, b), distance);
        edges.insert((b, a), distance);
    }

    let nodes = nodes
        .into_iter()
        .collect::<Vec<_>>();

    nodes.permutations()
        .map(|perm|
            perm
                .windows(2)
                .map(|w| edges.get(&(w[0], w[1])).unwrap())
                .sum()
        )
        .into_iter()
        .reduce(|a, b| f(a, b))
        .unwrap()
}

#[crate::aoc(year = 2015, day = 9, part = "A")]
fn day09a(lines: &[&str]) -> usize {
    day09(lines, usize::min)
}

#[crate::aoc(year = 2015, day = 9, part = "B")]
fn day09b(lines: &[&str]) -> usize {
    day09(lines, usize::max)
}

#[cfg(test)]
mod tests_y2015_day09 {
    use super::*;

    const LINES: &[&str; 3] = &[
        "London to Dublin = 464",
        "London to Belfast = 518",
        "Dublin to Belfast = 141",
    ];

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day09a(LINES), 605);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day09b(LINES), 982);
    }
}
