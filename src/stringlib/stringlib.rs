use std::{
    ops::Range,
    num::NonZeroU64
};

use pad::{
    PadStr,
    Alignment
};

pub const ASCII_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

pub const ASCII_UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

const UPPERCASE_RANGE: Range<u8> = 65..91;
const LOWERCASE_RANGE: Range<u8> = 97..123;

// const fn get_uppercase_len() -> usize {
//     uppercase_range.len()
// }

// const total_uppers: usize = get_uppercase_len();
// const total_lowers: usize = lowercase_range.len();

pub fn generate_ascii_alpha_lowercase() -> Vec<char> {
    LOWERCASE_RANGE.map(|index| index as char).collect()
}

pub fn generate_ascii_alpha_uppercase() -> Vec<char> {
    // const variable: i32 = 123;
    UPPERCASE_RANGE.map(|index| index as char).collect()
}

// const fn generate_ascii_alpha() -> Vec<char> {
//     let mut upper = generate_ascii_alpha_uppercase();
//     upper.extend(generate_ascii_alpha_lowercase());
//     upper
// }

// pub const ascii_uppercase: Vec<char> = generate_ascii_alpha_uppercase();
// pub const ascii_lowercase: Vec<char> = generate_ascii_alpha_lowercase();

pub fn align_center(
    _string: &str,
    width: usize
) -> String {
    _string.pad(width, ' ', Alignment::Middle, true) // truncate true
}

pub fn align_left(
    _string: &str,
    width: usize
) -> String {
    _string.pad(width, ' ', Alignment::Left, true) // truncate true
}

pub fn align_right(
    _string: &str,
    width: usize
) -> String {
    _string.pad(width, ' ', Alignment::Right, true) // truncate true
}

pub fn align_center_fillchar(
    _string: &str,
    width: usize,
    fillchar: char
) -> String {
    _string.pad(width, fillchar, Alignment::Middle, true) // truncate true
}

pub struct AlignString {
    content:          String,
    width:            Option<usize>,
    fillchar:         Option<char>,
    alignment:        Option<Alignment>,
    cached_formatted: Option<String>
}

impl AlignString {
    pub fn new(content: &str) -> Self {
        Self {
            content:          String::from(content),
            width:            None,
            fillchar:         None,
            alignment:        None,
            cached_formatted: None
        }
    }

    fn format(&mut self) {
        let width = match self.width {
            Some(w) => w,
            None => self.content.len()
        };

        let pad_char = self.fillchar.unwrap_or(' ');

        let alignment = match self.alignment {
            Some(al) => al,
            None => Alignment::Middle
        };

        let formatted = self.content.pad(width, pad_char, alignment, true);
        self.cached_formatted = Some(formatted)
    }

    pub fn width(
        &mut self,
        width: usize
    ) -> &mut Self {
        self.width = Some(width);
        self.format();
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.alignment = Some(Alignment::Left);
        self.format();
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.alignment = Some(Alignment::Right);
        self.format();
        self
    }

    pub fn center(&mut self) -> &mut Self {
        self.alignment = Some(Alignment::Middle);
        self.format();
        self
    }

    pub fn alignment(
        &mut self,
        alignment: Alignment
    ) -> &mut Self {
        self.alignment = Some(alignment);
        self.format();
        self
    }

    pub fn fillchar(
        &mut self,
        fillchar: char
    ) -> &mut Self {
        self.fillchar = Some(fillchar);
        self.format();
        self
    }

    #[rustfmt::skip]
    pub fn build(&self) -> String {
        let temp = self.cached_formatted.as_ref().unwrap_or(&self.content);
        String::from(temp)
    }
}

impl std::fmt::Display for AlignString {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        match self.cached_formatted {
            Some(ref cached) => f.write_str(cached),
            None => f.write_str(&self.content)
        }
    }
}
