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
    "weird"
];

#[derive(Debug)] // derive std::fmt::Debug on FontNotFoundError
pub struct FontNotFoundError {
    code:    usize,
    message: String
}

impl fmt::Display for FontNotFoundError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter
    ) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!"
        };

        write!(f, "{}", err_msg)
    }
}

type FontResult = Result<(), FontNotFoundError>;

fn throw_font_not_found_error() -> FontResult {
    Err(FontNotFoundError {
        code:    404,
        message: String::from("Page not found")
    })
}

type StringResult = Result<String, FontNotFoundError>;

#[deprecated = "the ascii website API: `http://artii.herokuapp.com` doesnt exist anymore"]
pub fn asciify(
    text: impl AsRef<str>,
    font: Option<String>
) -> StringResult {
    let text = text.as_ref();
    let font: String = font.unwrap_or("".to_string());

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
            throw_font_not_found_error()?;
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

use std::error;

#[derive(Debug)] // derive std::fmt::Debug on FondNotFoundError
pub struct BackSlashNMissingError {
    code:    usize,
    message: String
}

impl fmt::Display for BackSlashNMissingError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter
    ) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!"
        };

        write!(f, "{}", err_msg)
    }
}

fn throw_bask_slash_n_missing_error() -> Result<(), BackSlashNMissingError>
{
    Err(BackSlashNMissingError {
        code:    404,
        message: String::from("Page not found")
    })
}

fn shift_right_asciified(
    asciified_text: impl AsRef<str>,
    size: usize
) -> Result<String, Box<dyn error::Error>> {
    let asciified_text = asciified_text.as_ref();

    if !asciified_text.contains("\n") {
        throw_bask_slash_n_missing_error();
    }

    let result = asciified_text
        .split("\n")
        .map(|line| " ".repeat(size) + line)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(result)
}

fn shift_left_asciifiied_text(
    asciified_text: impl AsRef<str>,
    size: usize
) -> Result<String, Box<dyn error::Error>> {
    let asciified_text = asciified_text.as_ref();

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
    pub strikeout: &'ansi_lifetime str
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
        strikeout: "\x1b[9m"
    }
}

pub const ANSIColors: ANSITerm = initialize_ansi_term();

#[cfg(test)]
mod tests {
    use super::ANSIColors;

    #[test]
    fn test_asciify_str() {
        println!("'{}'asd asdasd{}", ANSIColors.red, ANSIColors.endc);
        assert_eq!(0, 0);
    }
}

struct RGBANSIEscapeCodesStruct {
    pub blue_ocean:    &'static str,
    pub yellow_dark:   &'static str,
    pub yellow_bright: &'static str,
    pub lime_green:    &'static str,
    pub blue_dark:     &'static str,
    pub red:           &'static str,
    pub cyan:          &'static str,
    pub blue:          &'static str,
    pub purple:        &'static str,
    pub magenta:       &'static str,
    pub orange:        &'static str,
    pub green_pine:    &'static str,
    pub gray:          &'static str,
    pub endc:          &'static str
}

const fn initialize_rgb_colors() -> RGBANSIEscapeCodesStruct {
    RGBANSIEscapeCodesStruct {
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
        endc:          "\x1b[0m"
    }
}

const RGB_COLORS: RGBANSIEscapeCodesStruct = initialize_rgb_colors();

pub enum RGBColorsEnum {
    Cyan(u8, u8, u8)
}

pub const RGBCyan: RGBColorsEnum = RGBColorsEnum::Cyan(101, 200, 179);

pub const ASCII_FERRIS_LOGO: &'static str = r#"
█ █         █ █
▀█  ▄█████▄  █▀
 ▀▄███▀█▀███▄▀
 ▄▀███▀▀▀███▀▄
 █ ▄▀▀▀▀▀▀▀▄ █
"#;

// ferris animation frames
// <body>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '-,   -  _'\
// | '----'
// </pre>
// <pre>
//
// .~'^'^-, (/
// \) /  o O  |'
// '-,   -  _'\
// | '----'
// </pre>
// <pre>
//
// .~'^'^-, (/
// \) /  o O  |'
// '-,   -  _'\
// | '----'
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '-,   -  _'\
// | '----'
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \/ /  o o  \ \/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \/ /  o o  \ \/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \/ /  o o  \ \/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// /'_  -   ,-'
// '----' |
// </pre>
// <pre>
//
// \) ,-^'^'~.
// '|  O o  \ (/
// /'_  -   ,-'
// '----' |
// </pre>
// <pre>
//
// \) ,-^'^'~.
// '|  O o  \ (/
// /'_  -   ,-'
// '----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// /'_  -   ,-'
// '----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// / '-----' \
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// | '-----' |
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \/ /  o o  \ \/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \/ /  o o  \ \/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \/ /  o o  \ \/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   u   _'
// \ '-----' /
// </pre>
// <pre>
//
// _~^~^~_
// \) /  o o  \ (/
// '_   ¬   _'
// \ '-----' /
// </pre>
// </body>
