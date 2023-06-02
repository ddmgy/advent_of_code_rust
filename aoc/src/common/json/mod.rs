pub mod error;
pub(crate) mod scanner;

use std::collections::HashMap;

use error::Error;
use scanner::{Scanner, Token};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Json {
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
    Number(i64),
    String(String),
}

impl Json {
    pub fn parse(input: &[u8]) -> Result<Json, Error> {
        let (token, _, _) = Scanner::scan(input)?;

        Self::map_token_to_json(token)
    }

    fn map_token_to_json(token: Token) -> Result<Json, Error> {
        match token {
            Token::Array(tokens) => {
                let mut arr = vec![];

                for token in tokens {
                    arr.push(Self::map_token_to_json(token)?);
                }

                Ok(Self::Array(arr))
            },
            Token::Object(kvs) => {
                let mut map = HashMap::new();

                for (key, value) in kvs {
                    let key = match key {
                        Token::String(bytes) => String::from_utf8(bytes.to_vec())?,
                        _ => return Err(Error::InvalidObjectKey),
                    };

                    map.insert(key, Json::map_token_to_json(value)?);
                }

                Ok(Self::Object(map))
            },
            Token::Number(bytes) => {
                let (sign, start) = match bytes[0] {
                    b'-' => (-1i64, 1usize),
                    _ => (1, 0),
                };
                let mut curr = 0;

                for &c in &bytes[start..] {
                    curr = (curr * 10) + ((c - b'0') as i64);
                }

                Ok(Self::Number(curr * sign))
            },
            Token::String(bytes) => {
                Ok(Self::String(String::from_utf8(bytes.to_vec())?))
            },
        }
    }
}

macro_rules! impl_json_number {
    (__impl $ty:ty) => {
        impl From<$ty> for Json {
            fn from(value: $ty) -> Self {
                Json::Number(value as i64)
            }
        }
    };
    ($ty:ty) => {
        impl_json_number!(__impl $ty);
    };
    ($ty:ty, $($tys:ty),+) => {
        impl_json_number!(__impl $ty);
        impl_json_number!($($tys),+);
    };
}

impl_json_number!(isize, i8, i16, i32, i64, i128);
impl_json_number!(usize, u8, u16, u32, u64, u128);

macro_rules! impl_json_string {
    (__impl $ty:ty) => {
        impl From<$ty> for Json {
            fn from(value: $ty) -> Self {
                Json::String(String::from(value))
            }
        }
    };
    ($ty:ty) => {
        impl_json_string!(__impl $ty);
    };
    ($ty:ty, $($tys:ty),+) => {
        impl_json_string!(__impl $ty);
        impl_json_string!($($tys),+);
    };
}

impl_json_string!(String, &str, &String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_parse() {
        let _json = Json::Array(vec![
            Json::Number(1),
            Json::Number(2),
            Json::Number(3),
        ]);

        let mut map = HashMap::new();
        map.insert(String::from("a"), Json::Number(2));
        map.insert(String::from("b"), Json::Number(4));
        let _json = Json::Object(map);

        let _json = Json::Array(vec![
            Json::Array(vec![
                Json::Array(vec![
                    Json::Number(3),
                ]),
            ]),
        ]);

        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        map2.insert(String::from("b"), Json::Number(4));
        map1.insert(String::from("a"), Json::Object(map2));
        map1.insert(String::from("c"), Json::Number(-1));
        let _json = Json::Object(map1);

        let mut map = HashMap::new();
        map.insert(String::from("a"), Json::Array(vec![
            Json::Number(-1),
            Json::Number(1),
        ]));
        let _json = Json::Object(map);

        let mut map = HashMap::new();
        map.insert(String::from("a"), Json::Number(1));
        let _json = Json::Array(vec![
            Json::Number(-1),
            Json::Object(map),
        ]);
    }

    #[test]
    fn json_parse_number() {
        let json = Json::parse(b"123");
        assert_eq!(json, Ok(Json::Number(123)));

        let json = Json::parse(b"-91");
        assert_eq!(json, Ok(Json::Number(-91)));
    }

    #[test]
    fn json_parse_string() {
        let json = Json::parse(br#""red""#);
        assert_eq!(json, Ok(Json::String(String::from("red"))));

        let json = Json::parse(br#""""#);
        assert_eq!(json, Ok(Json::String(String::from(""))));

        let json = Json::parse(br#""a string with spaces""#);
        assert_eq!(json, Ok(Json::String(String::from("a string with spaces"))));
    }

    #[test]
    fn json_parse_array() {
        let json = Json::parse(b"[[[3]]]");
        assert_eq!(json, Ok(
            Json::Array(vec![
                Json::Array(vec![
                    Json::Array(vec![
                        Json::Number(3),
                    ]),
                ]),
            ])));
    }

    #[test]
    fn json_parse_object() {
        let json = Json::parse(br#"{"red":[200]}"#);
        let mut map = HashMap::new();
        map.insert(String::from("red"), Json::Array(vec![Json::Number(200)]));
        assert_eq!(json, Ok(Json::Object(map)));
    }
}
