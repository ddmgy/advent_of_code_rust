use crate::common::Json;

fn count_numbers(json: Json, is_part_b: bool) -> i64 {
    let mut count = 0;

    match json {
        Json::Number(n) => count += n,
        Json::String(_) => {},
        Json::Object(map) => {
            if is_part_b {
                for value in map.values() {
                    match value {
                        Json::String(s) => {
                            if s == "red" {
                                return 0;
                            }
                        },
                        _ => {},
                    }
                }
            }

            for (_, j) in map {
                count += count_numbers(j, is_part_b);
            }
        },
        Json::Array(arr) => {
            for j in arr {
                count += count_numbers(j, is_part_b);
            }
        },
    }

    count
}

fn day12(input: &[u8], is_part_b: bool) -> i64 {
    match Json::parse(input) {
        Ok(json) => count_numbers(json, is_part_b),
        Err(e) => {
            eprintln!("{:?}", e);
            -1
        },
    }
}

#[crate::aoc(year = 2015, day = 12, part = "A")]
fn day12a(input: &[u8]) -> i64 {
    day12(input, false)
}

#[crate::aoc(year = 2015, day = 12, part = "B")]
fn day12b(input: &[u8]) -> i64 {
    day12(input, true)
}

#[cfg(test)]
mod tests_y2015_day12 {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn partA() {
        assert_eq!(day12a(b"[1,2,3]"), 6);
        assert_eq!(day12a(br#"{"a":2,"b":4}"#), 6);

        assert_eq!(day12a(b"[[[3]]]"), 3);
        assert_eq!(day12a(br#"{"a":{"b":4},"c":-1}"#), 3);

        assert_eq!(day12a(br#"{"a":[-1,1]}"#), 0);
        assert_eq!(day12a(br#"[-1,{"a":1}]"#), 0);

        assert_eq!(day12a(b"[]"), 0);
        assert_eq!(day12a(b"{}"), 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn partB() {
        assert_eq!(day12b(b"[1,2,3]"), 6);
        assert_eq!(day12b(br#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(day12b(br#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(day12b(br#"[1,"red",5]"#), 6);
    }
}
