use crate::common::CombinationsExt;

fn day17(containers: Vec<usize>, eggnog: usize, is_part_2: bool) -> usize {
    let mut containers = containers;
    containers.sort();
    let mut count = 0;
    let mut max_len = 0;
    let mut sum = 0;

    for container in &containers {
        sum += container;
        if sum >= eggnog {
            break;
        }

        max_len += 1;
    }

    for n in 2..=max_len {
        let mut found = false;
        let containers = containers.clone();
        for comb in containers.into_iter().combinations(n) {
            if comb.iter().sum::<usize>() == eggnog {
                count += 1;
                found = true;
            }
        }

        if is_part_2 && found {
            break;
        }
    }

    count
}


#[crate::aoc(year = 2015, day = 17, part = "A")]
fn day17a(containers: Vec<usize>) -> usize {
    day17(containers, 150, false)
}

#[crate::aoc(year = 2015, day = 17, part = "B")]
fn day17b(containers: Vec<usize>) -> usize {
    day17(containers, 150, true)
}

#[cfg(test)]
mod tests_y2015_day17 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        let INPUT = vec![20, 15, 10, 5, 5];
        assert_eq!(day17(INPUT, 25, false), 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        let INPUT = vec![20, 15, 10, 5, 5];
        assert_eq!(day17(INPUT, 25, true), 3);
    }
}
