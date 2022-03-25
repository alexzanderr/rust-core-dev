//! some quick docs about this module
//!
//! src/aesthetics.rs
//!
//! useful and fancy design in terminal
//!
//! here you can
//!     - asciify text
//!     - color asciified text
//!     - color normal text
//!
//! author: @alexzanderr
//!
//!
//!

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    unused_must_use,
    non_snake_case,
    non_upper_case_globals
)]


use std::fmt;

// https://stackoverflow.com/questions/58906965/could-not-find-blocking-in-reqwest
use reqwest as request;


/// just a cool constant
const IMPRESSIVE_FONTS: [&str; 29] = [
    "bell",
    "big",
    "broadway",
    "bubble",
    "chunky",
    "contessa",
    "cursive",
    "cyberlarge",
    "cybermedun",
    "cybersmall",
    "digital",
    "doh",
    "doom",
    "double",
    "drpepper",
    "epic",
    "fender",
    "kban",
    "l4me",
    "larry3d",
    "ogre",
    "rectangles",
    "shadow",
    "slant",
    "small",
    "smkeyboard",
    "speed",
    "standard",
    "weird",
];

#[derive(Debug)] // derive std::fmt::Debug on FontNotFoundError
pub struct FontNotFoundError {
    code:    usize,
    message: String,
}


impl fmt::Display for FontNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

type FontResult = Result<(), FontNotFoundError>;

fn throw_font_not_found_error() -> FontResult {
    Err(FontNotFoundError {
        code:    404,
        message: String::from("Page not found"),
    })
}


enum Strings<'StringsLifetime> {
    Static(&'StringsLifetime str),
    String(String),
}

type StringResult = Result<String, FontNotFoundError>;

fn _asciify(_text: Strings, _font: Option<Strings>) -> StringResult {
    let text: String = match _text {
        Strings::Static(_text) => String::from(_text),
        Strings::String(_text) => _text,
    };
    let font: String = match _font {
        Some(_font) => match _font {
            Strings::Static(_font) => String::from(_font),
            Strings::String(_font) => _font,
        },
        None => String::from(""),
    };

    let mut font_argument = String::from("");
    if font != "" {
        let mut found: bool = false;
        for _font in IMPRESSIVE_FONTS {
            if _font == font {
                found = true;
                break;
            }
        }
        if !found {
            throw_font_not_found_error();
        }
        font_argument = format!("&font={font}");
    }

    const ASCIIFY_BASE_URL: &str = "http://artii.herokuapp.com";

    let formatted_text: Vec<&str> = text.split(' ').collect();
    let formatted_text = formatted_text.join("+");
    let url_parameters =
        format!("/make?text={formatted_text}{font_argument}");

    let url = format!("{ASCIIFY_BASE_URL}{url_parameters}");
    let response = request::blocking::get(url).unwrap();
    let asciifyied_text = response.text().unwrap();

    Ok(asciifyied_text)
}

/// puts 2 lines of spaces after the modified text.
pub fn asciify_str(
    _text: &str,
    _font: Option<&str>,
) -> Result<String, FontNotFoundError> {
    let text = Strings::Static(_text);
    let font: Option<Strings> = match _font {
        Some(_font) => Some(Strings::Static(_font)),
        None => None,
    };
    _asciify(text, font)
}


/// puts 2 lines of spaces after the modified text.
pub fn asciify_string(
    _text: String,
    _font: Option<String>,
) -> Result<String, FontNotFoundError> {
    let text = Strings::String(_text);
    let font: Option<Strings> = match _font {
        Some(_font) => Some(Strings::String(_font)),
        None => None,
    };
    _asciify(text, font)
}

use std::error;


#[derive(Debug)] // derive std::fmt::Debug on FondNotFoundError
pub struct BackSlashNMissingError {
    code:    usize,
    message: String,
}


impl fmt::Display for BackSlashNMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

fn throw_bask_slash_n_missing_error() -> Result<(), BackSlashNMissingError>
{
    Err(BackSlashNMissingError {
        code:    404,
        message: String::from("Page not found"),
    })
}

fn _shift_right_asciified(
    _asciified_text: Strings,
    size: usize,
) -> Result<String, Box<dyn error::Error>> {
    let asciified_text: String = match _asciified_text {
        Strings::Static(_asciified_text) => String::from(_asciified_text),
        Strings::String(_asciified_text) => _asciified_text,
    };

    if !asciified_text.contains("\n") {
        throw_bask_slash_n_missing_error();
    }

    let result: String = asciified_text
        .split("\n")
        .map(|line| " ".repeat(size) + line)
        .collect::<Vec<String>>()
        .join("\n");


    Ok(result)
}

pub fn shift_right_asciified_str(
    _text: &str,
    size: usize,
) -> Result<String, Box<dyn error::Error>> {
    let text = Strings::Static(_text);
    _shift_right_asciified(text, size)
}

pub fn shift_right_asciified_string(
    _text: String,
    size: usize,
) -> Result<String, Box<dyn error::Error>> {
    let text = Strings::String(_text);
    _shift_right_asciified(text, size)
}


fn _shift_left_asciifiied_text(
    _asciified_text: Strings,
    size: usize,
) -> Result<String, Box<dyn error::Error>> {
    let asciified_text: String = match _asciified_text {
        Strings::Static(_asciified_text) => String::from(_asciified_text),
        Strings::String(_asciified_text) => _asciified_text,
    };

    if !asciified_text.contains("\n") {
        throw_bask_slash_n_missing_error();
    }

    let result: String = asciified_text
        .split("\n")
        .map(|line| &line[size..])
        .collect::<Vec<&str>>()
        .join("\n");


    Ok(result)
}


pub struct ANSITerm<'ansi_lifetime> {
    pub black:     &'ansi_lifetime str,
    pub magenta:   &'ansi_lifetime str,
    pub cyan:      &'ansi_lifetime str,
    pub white:     &'ansi_lifetime str,
    pub red:       &'ansi_lifetime str,
    pub green:     &'ansi_lifetime str,
    pub yellow:    &'ansi_lifetime str,
    pub blue:      &'ansi_lifetime str,
    pub purple:    &'ansi_lifetime str,
    pub erase:     &'ansi_lifetime str,
    pub bold:      &'ansi_lifetime str,
    pub italic:    &'ansi_lifetime str,
    pub underline: &'ansi_lifetime str,
    pub endc:      &'ansi_lifetime str,
    pub inverse:   &'ansi_lifetime str,
    pub strikeout: &'ansi_lifetime str,
}

const fn initialize_ansi_term() -> ANSITerm<'static> {
    ANSITerm {
        black:     "\x1b[30m",
        magenta:   "\x1b[35m",
        cyan:      "\x1b[36m",
        white:     "\x1b[37m",
        red:       "\x1b[91m",
        green:     "\x1b[92m",
        yellow:    "\x1b[93m",
        blue:      "\x1b[94m",
        purple:    "\x1b[95m",
        erase:     "\x1b[8m",
        bold:      "\x1b[1m",
        italic:    "\x1b[3m",
        underline: "\x1b[4m",
        endc:      "\x1b[0m",
        inverse:   "\x1b[7m",
        strikeout: "\x1b[9m",
    }
}

pub const ansi: ANSITerm = initialize_ansi_term();


#[cfg(test)]
mod tests {
    use super::ansi;


    #[test]
    fn test_asciify_str() {
        println!("'{}'asd asdasd{}", ansi.red, ansi.endc);
        assert_eq!(0, 0);
    }
}


struct RGBColorsStruct {
    blue_ocean:    &'static str,
    yellow_dark:   &'static str,
    yellow_bright: &'static str,
    lime_green:    &'static str,
    blue_dark:     &'static str,
    red:           &'static str,
    cyan:          &'static str,
    blue:          &'static str,
    purple:        &'static str,
    magenta:       &'static str,
    orange:        &'static str,
    green_pine:    &'static str,
    gray:          &'static str,
    endc:          &'static str,
}

const fn initialize_rgb_colors() -> RGBColorsStruct {
    RGBColorsStruct {
        blue_ocean:    "\x1b[38;2;38;199;186m",
        yellow_dark:   "\x1b[38;2;200;179;1m",
        yellow_bright: "\x1b[38;2;230;186;52m",
        lime_green:    "\x1b[38;2;168;230;52m",
        blue_dark:     "\x1b[38;2;38;129;199m",
        red:           "\x1b[38;2;205;70;77m",
        cyan:          "\x1b[38;2;101;200;179m",
        blue:          "\x1b[38;2;70;142;205m",
        purple:        "\x1b[38;2;186;70;205m",
        magenta:       "\x1b[38;2;205;70;164m",
        orange:        "\x1b[38;2;230;114;52m",
        green_pine:    "\x1b[38;2;38;199;87m",
        gray:          "\x1b[38;2;108;108;108m",
        endc:          "\x1b[0m",
    }
}

pub enum RGBColorsEnum {
    Cyan(u8, u8, u8),
}

pub const RGBCyan: RGBColorsEnum = RGBColorsEnum::Cyan(101, 200, 179);
