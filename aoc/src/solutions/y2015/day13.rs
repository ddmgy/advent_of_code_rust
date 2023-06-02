use std::collections::{HashMap, HashSet};

use crate::common::PermutationsExt;

fn day13(lines: &[&str], is_part_b: bool) -> i64 {
    let mut changes = HashMap::new();
    let mut names = HashSet::new();

    for &line in lines {
        let parts: Vec<_> = line.split(" ").collect();
        let n1 = parts[0];
        let n2 = parts[10].trim_end_matches(|c| c == '.');
        let change = parts[3].parse::<i64>().unwrap()
            * if parts[2] == "lose" { -1 } else { 1 };

        changes.insert((n1, n2), change);
        names.insert(n1);
    }

    if is_part_b {
        for &name in &names {
            changes.insert(("me", name), 0);
            changes.insert((name, "me"), 0);
        }

        names.insert("me");
    }

    let names: Vec<_> = names.into_iter().collect();
    let len = names.len();

    names.permutations()
        .map(|perm| {
            let mut total = 0;

            for i in 0..len {
                let prev_i = if i == 0 {
                    len - 1
                } else {
                    i - 1
                };
                let next_i = if i == len - 1 {
                    0
                } else {
                    i + 1
                };
                let prev = perm[prev_i];
                let name = perm[i];
                let next = perm[next_i];

                if let Some(change) = changes.get(&(name, prev)) {
                    total += change;
                } else {
                    println!("unknown happiness change: ({}, {})", name, prev);
                }

                if let Some(change) = changes.get(&(name, next)) {
                    total += change;
                } else {
                    println!("unknown happiness change: ({}, {})", name, next);
                }
            }

            total
        })
        // .take(2)
        .max()
        .unwrap()
}

#[crate::aoc(year = 2015, day = 13, part = "A")]
fn day13a(lines: &[&str]) -> i64 {
    day13(lines, false)
}

#[crate::aoc(year = 2015, day = 13, part = "B")]
fn day13b(lines: &[&str]) -> i64 {
    day13(lines, true)
}

#[cfg(test)]
mod tests_y2015_day13 {
    use super::*;

    const SEATING_ARRANGEMENT: &[&str] = &[
        "Alice would gain 54 happiness units by sitting next to Bob.",
        "Alice would lose 79 happiness units by sitting next to Carol.",
        "Alice would lose 2 happiness units by sitting next to David.",
        "Bob would gain 83 happiness units by sitting next to Alice.",
        "Bob would lose 7 happiness units by sitting next to Carol.",
        "Bob would lose 63 happiness units by sitting next to David.",
        "Carol would lose 62 happiness units by sitting next to Alice.",
        "Carol would gain 60 happiness units by sitting next to Bob.",
        "Carol would gain 55 happiness units by sitting next to David.",
        "David would gain 46 happiness units by sitting next to Alice.",
        "David would lose 7 happiness units by sitting next to Bob.",
        "David would gain 41 happiness units by sitting next to Carol.",
    ];

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day13a(SEATING_ARRANGEMENT), 330);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
    }
}
