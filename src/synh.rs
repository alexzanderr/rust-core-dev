use syntect::easy::HighlightLines;
use syntect::parsing::{
    SyntaxSet,
    SyntaxReference,
};
use syntect::highlighting::{
    ThemeSet,
    Style,
    Theme,
};
use syntect::util::{
    as_24_bit_terminal_escaped,
    LinesWithEndings,
};

pub struct Highlighter<'a> {
    pub theme_name: &'a str,
    theme:          Theme,
    pub theme_file: &'a str,
    syntax_set:     SyntaxSet,
    language:       &'a str, // hl: HighlightLines<'a>
}

impl<'a> Highlighter<'a> {
    pub fn new(theme_file: &'a str, language: &'a str) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();

        let tm_path = std::path::Path::new(theme_file);
        let theme = ThemeSet::get_theme(tm_path).unwrap();

        Self {
            theme_name: "asd",
            theme_file,
            syntax_set,
            theme,
            language,
        }
    }

    pub fn set_theme(&mut self, theme_file: &'a str) {
        let tm_path = std::path::Path::new(theme_file);
        self.theme = ThemeSet::get_theme(tm_path).unwrap()
    }

    pub fn set_language(&'a mut self, language: &'a str) {
        self.language = language;
    }

    pub fn highlight_line(&self, line: &str) -> String {
        let s = self
            .syntax_set
            .find_syntax_by_extension(self.language)
            .unwrap();
        let mut hl = HighlightLines::new(s, &self.theme);
        let ranges: Vec<(Style, &str)> =
            hl.highlight(line, &self.syntax_set);
        as_24_bit_terminal_escaped(&ranges[..], false) + "\x1b[0m"
    }

    pub fn highlight_lines(
        &self,
        lines: Vec<&'a str>,
    ) -> Vec<String> {
        let s = self
            .syntax_set
            .find_syntax_by_extension(self.language)
            .unwrap();
        let mut hl = HighlightLines::new(s, &self.theme);

        lines
            .iter()
            .map(|line| {
                let ranges: Vec<(Style, &str)> =
                    hl.highlight(line, &self.syntax_set);
                as_24_bit_terminal_escaped(&ranges[..], false)
            })
            .collect()
    }

    pub fn highlighting_string(self, _string: &'a str) -> String {
        let s = self
            .syntax_set
            .find_syntax_by_extension(self.language)
            .unwrap();
        let mut hl = HighlightLines::new(s, &self.theme);

        let ranges: Vec<(Style, &str)> =
            hl.highlight(_string, &self.syntax_set);
        as_24_bit_terminal_escaped(&ranges[..], false)
    }
}

pub fn highlight_line(
    line: &str,
    theme_file: &str,
    language: &str,
) -> String {
    let ps = SyntaxSet::load_defaults_newlines();

    let tm_path = std::path::Path::new(theme_file);
    let theme = ThemeSet::get_theme(tm_path).unwrap();
    // None => {
    //     let mut contents =
    //         std::io::Cursor::new(SOLARIZED_DARK_THEME.as_bytes());
    //     ThemeSet::load_from_reader(&mut contents).unwrap()
    // },

    let syntax = ps.find_syntax_by_extension(language).unwrap();
    let mut highligther = HighlightLines::new(syntax, &theme);

    let ranges: Vec<(Style, &str)> = highligther.highlight(line, &ps);
    as_24_bit_terminal_escaped(&ranges[..], false)
}
