use crate::traits::StringExtended;
use std::fmt::Write as FmtWrite;

#[derive(Default)]
pub struct MultiLineString {
    lines:         Vec<String>,
    cached_string: String
}

impl MultiLineString {
    // TODO
    // cached_string has extra \n at the end -> problem
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_line(
        &mut self,
        index: usize,
        line: impl AsRef<str>
    ) {
        let line = line.as_ref();
        self.lines.insert(index, String::from(line));

        let mut cached_lines = self.cached_string.split_to_vec_str("\n");
        cached_lines.insert(index, line);
        cached_lines.insert(index, "\n");
        self.cached_string = cached_lines.join("\n");

        // self.cached_string.push_str(line);
        // self.cached_string.push_str("\n");
    }

    pub fn append_line(
        &mut self,
        line: impl AsRef<str>
    ) {
        let line = line.as_ref();
        self.lines.push(String::from(line));
        writeln!(self.cached_string, "{}", line);
    }

    fn insert_token(
        &mut self,
        token: &str
    ) {
        for line in self.lines.iter_mut() {
            *line = format!("{}{}", token, line);
        }

        let mut cached_lines =
            self.cached_string.split_to_vec_string("\n");
        for cached_line in cached_lines.iter_mut() {
            *cached_line = format!("{}{}", token, cached_line);
        }
        self.cached_string = cached_lines.join("\n");
    }

    pub fn indent_with_token(
        &mut self,
        token: &str
    ) {
        self.insert_token(token);
    }

    pub fn indent_with_spaces(
        &mut self,
        total: usize
    ) {
        let token = " ".repeat(total);
        self.insert_token(&token);
    }
}

impl std::fmt::Display for MultiLineString {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        f.write_str(&self.cached_string)
    }
}
