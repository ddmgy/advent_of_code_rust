use crate::common::CombinationsExt;

fn day24(packages: Vec<u64>, num_compartments: u64) -> Option<u64> {
    let mut result = u64::MAX;
    let compartment_weight = packages.clone().into_iter().sum::<u64>() / num_compartments;

    for i in 1..(packages.len() - num_compartments as usize + 1) {
        for comb in packages.iter().combinations(i) {
            if comb.clone().into_iter().sum::<u64>() == compartment_weight {
                result = std::cmp::min(result, comb.into_iter().product());
            }
        }

        if result < u64::MAX {
            break;
        }
    }

    Some(result)
}

#[crate::aoc(year = 2015, day = 24, part = "A")]
fn day24a(input: Vec<u64>) -> Option<u64> {
    day24(input, 3)
}

#[crate::aoc(year = 2015, day = 24, part = "B")]
fn day24b(input: Vec<u64>) -> Option<u64> {
    day24(input, 4)
}

#[cfg(test)]
mod tests_y2015_day24 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        let test_packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(day24a(test_packages), Some(99));
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        let test_packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(day24b(test_packages), Some(44));
    }
}
