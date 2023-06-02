use super::error::Error;

#[derive(Debug)]
pub(crate) enum Token<'a> {
    Array(Vec<Token<'a>>),
    Object(Vec<(Token<'a>, Token<'a>)>),
    Number(&'a [u8]),
    String(&'a [u8]),
}

pub(crate) struct Scanner;

impl Scanner {
    pub(crate) fn scan(input: &[u8]) -> Result<(Token, &[u8], usize), Error> {
        match input.get(0) {
            None => return Err(Error::UnexpectedEndOfInput),
            Some(b'[') => {
                let (slice, remainder, end) = Scanner::scan_array(input)?;
                let construct = Scanner::construct_array_from_slice(slice)?;

                Ok((construct, remainder, end))
            },
            Some(b'{') => {
                let (slice, remainder, end) = Scanner::scan_object(input)?;
                let construct = Scanner::construct_object_from_slice(slice)?;

                Ok((construct, remainder, end))
            },
            Some(b'-' | b'0'..=b'9') => {
                let (slice, remainder, end) = Scanner::scan_number(input)?;

                Ok((Token::Number(slice), remainder, end))
            },
            Some(b'"') => {
                let (slice, remainder, end) = Scanner::scan_string(input)?;

                Ok((Token::String(slice), remainder, end))
            },
            Some(c) => return Err(Error::UnexpectedChar(*c as char)),
        }
    }

    #[inline(always)]
    fn scan_array(input: &[u8]) -> Result<(&[u8], &[u8], usize), Error> {
        Scanner::scan_compound(input, b'[', b']')
    }

    #[inline(always)]
    fn scan_object(input: &[u8]) -> Result<(&[u8], &[u8], usize), Error> {
        Scanner::scan_compound(input, b'{', b'}')
    }

    fn scan_compound(input: &[u8], inc: u8, dec: u8) -> Result<(&[u8], &[u8], usize), Error> {
        let mut depth = 0;

        for (i, &c) in input.iter().enumerate() {
            match c {
                c if c == inc => depth += 1,
                c if c == dec => {
                    depth -= 1;
                    if depth == 0 {
                        let (slice, remainder) = input.split_at(i + 1);
                        return Ok((slice, remainder, i + 1))
                    } else if depth < 0 {
                        return Err(Error::UnbalancedBracket(c as char));
                    }
                },
                _ => {},
            }
        }

        Err(Error::UnexpectedEndOfInput)
    }

    fn scan_number(input: &[u8]) -> Result<(&[u8], &[u8], usize), Error> {
        let mut i = 0usize;

        for &c in input {
            match c {
                b'-' | b'0'..=b'9' => {},
                _ => break,
            }

            i += 1;
        }

        let (slice, remainder) = input.split_at(i);
        Ok((slice, remainder, i))
    }

    fn scan_string(input: &[u8]) -> Result<(&[u8], &[u8], usize), Error> {
        for (i, &c) in input.iter().enumerate().skip(1) {
            match c {
                b'"' => {
                    let slice = &input[1..i];
                    let remainder = &input[i + 1..];
                    return Ok((slice, remainder, i + 1))
                },
                _ => {},
            }
        }

        Err(Error::UnexpectedEndOfInput)
    }

    fn scan_char(input: &[u8], char: u8) -> Result<(&[u8], usize), Error> {
        match input.get(0) {
            Some(&_c) if _c == char => Ok((&input[1..], 1)),
            Some(_) => Ok((input, 0)),
            None => Err(Error::UnexpectedEndOfInput),
        }
    }

    fn construct_array_from_slice(slice: &[u8]) -> Result<Token, Error> {
        let mut arr = vec![];
        let mut slice = &slice[1..slice.len() - 1];

        while !slice.is_empty() {
            let (token, remainder, _) = Scanner::scan(slice)?;
            arr.push(token);

            match Scanner::scan_char(remainder, b',') {
                Ok((remainder, _)) => slice = remainder,
                Err(_) => break,
            }
        }

        Ok(Token::Array(arr))
    }

    fn construct_object_from_slice(slice: &[u8]) -> Result<Token, Error> {
        let mut elems = vec![];
        let mut slice = &slice[1..slice.len() - 1];

        while !slice.is_empty() {
            let (key, remainder, _) = Scanner::scan(slice)?;
            let (remainder, _) = Scanner::scan_char(remainder, b':')?;
            let (value, remainder, _) = Scanner::scan(remainder)?;
            elems.push((key, value));

            match Scanner::scan_char(remainder, b',') {
                Ok((remainder, _)) => slice = remainder,
                Err(_) => break,
            }
        }

        Ok(Token::Object(elems))
    }
}
