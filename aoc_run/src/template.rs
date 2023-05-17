use askama::Template;

#[derive(Template)]
#[template(path = "aoc.template", escape = "none", whitespace = "suppress")]
pub(crate) struct AocTemplate {
    year: usize,
    day: usize,
}

impl AocTemplate {
    pub(crate) fn new(year: usize, day: usize) -> Self {
        Self {
            year,
            day,
        }
    }
}
