use askama::Template;

#[derive(Template)]
#[template(path = "aoc.template", escape = "none", whitespace = "suppress")]
pub(crate) struct AocTemplate<'a> {
    year: usize,
    day: usize,
    input_name: &'a str,
    input_type: &'a str,
}

impl<'a> AocTemplate<'a> {
    pub(crate) fn new(year: usize, day: usize, input_name: &'a str, input_type: &'a str) -> Self {
        Self {
            year,
            day,
            input_name,
            input_type,
        }
    }
}
