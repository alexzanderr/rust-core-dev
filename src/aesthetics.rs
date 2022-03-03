//!
//! some quick docs about this module
//!
//! core/aesthetics.py
//!
//! useful and fancy design in terminal
//!
//! here you can
//!     - asciify text
//!     - color asciified text
//!     - color normal text
//!
//! author: @alexzander
//!
//!
//!
//!


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

#[derive(Debug)] // derive std::fmt::Debug on FondNotFoundError
pub struct FontNotFoundError {
    code: usize,
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

fn throw_font_not_found_error()
    -> Result<(), FontNotFoundError> {
    Err(FontNotFoundError {
        code: 404,
        message: String::from("Page not found"),
    })
}



enum Strings<'enum_lifetime> {
    Static(&'enum_lifetime str),
    String(String)
}

fn _asciify(
    _text: Strings,
    _font: Option<Strings>
) -> Result<String, FontNotFoundError>
{
    let text: String = match _text {
        Strings::Static(_text) => String::from(_text),
        Strings::String(_text) => _text
    };
    let font: String = match _font {
        Some(_font) => {
            match _font {
                Strings::Static(_font) => String::from(_font),
                Strings::String(_font) => _font
            }
        },
        None => {
            String::from("")
        }
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
    let url_parameters = format!("/make?text={formatted_text}{font_argument}");

    let url = format!("{ASCIIFY_BASE_URL}{url_parameters}");
    let response = request::blocking::get(url).unwrap();
    let asciifyied_text = response.text().unwrap();

    Ok(asciifyied_text)
}

/// puts 2 lines of spaces after the modified text.
pub fn asciify_str(
    _text: &str,
    _font: Option<&str>
) -> Result<String, FontNotFoundError>
{
    let text = Strings::Static(_text);
    let font: Option<Strings> = match _font {
        Some(_font) => Some(Strings::Static(_font)),
        None => None
    };
    _asciify(text, font)
}


/// puts 2 lines of spaces after the modified text.
pub fn asciify_string(
    _text: String,
    _font: Option<String>
) -> Result<String, FontNotFoundError>
{
    let text = Strings::String(_text);
    let font: Option<Strings> = match _font {
        Some(_font) => Some(Strings::String(_font)),
        None => None
    };
    _asciify(text, font)
}

use std::error;


#[derive(Debug)] // derive std::fmt::Debug on FondNotFoundError
pub struct BackSlashNMissingError {
    code: usize,
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

fn throw_bask_slash_n_missing_error()
    -> Result<(), BackSlashNMissingError> {
    Err(BackSlashNMissingError {
        code: 404,
        message: String::from("Page not found"),
    })
}

fn _shift_right_asciified(
    _asciified_text: Strings,
    size: usize
) -> Result<String, Box<dyn error::Error>>
{
    let asciified_text: String = match _asciified_text {
        Strings::Static(_asciified_text) =>
            String::from(_asciified_text),
        Strings::String(_asciified_text) =>
            _asciified_text
    };

    if !asciified_text.contains("\n") {
        throw_bask_slash_n_missing_error();
    }

    let result: String = asciified_text.split("\n")
        .map(|line| " ".repeat(size) + line)
        .collect::<Vec<String>>()
        .join("\n");


    Ok(result)
}

pub fn shift_right_asciified_str(
    _text: &str,
    size: usize
) -> Result<String, Box<dyn error::Error>>
{
    let text = Strings::Static(_text);
    _shift_right_asciified(text, size)
}

pub fn shift_right_asciified_string(
    _text: String,
    size: usize
) -> Result<String, Box<dyn error::Error>>
{
    let text = Strings::String(_text);
    _shift_right_asciified(text, size)
}



fn _shift_left_asciified(
    _asciified_text: Strings,
    size: usize
) -> Result<String, Box<dyn error::Error>>
{
    let asciified_text: String = match _asciified_text {
        Strings::Static(_asciified_text) =>
            String::from(_asciified_text),
        Strings::String(_asciified_text) =>
            _asciified_text
    };

    if !asciified_text.contains("\n") {
        throw_bask_slash_n_missing_error();
    }

    let result: String = asciified_text.split("\n")
        .map(|line| { &line[size..] })
        .collect::<Vec<&str>>()
        .join("\n");


    Ok(result)
}


pub struct Ansi {
    pub black: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
    pub white: &'static str,
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub purple: &'static str,
    pub erase: &'static str,
    pub bold: &'static str,
    pub italic: &'static str,
    pub underline: &'static str,
    pub endc: &'static str,
    pub inverse: &'static str,
    pub strikeout: &'static str,
}

const fn get_ansi() -> Ansi {
    Ansi {
        black: "\x1b[30m",
        magenta: "\x1b[35m",
        cyan: "\x1b[36m",
        white: "\x1b[37m",
        red: "\x1b[91m",
        green: "\x1b[92m",
        yellow: "\x1b[93m",
        blue: "\x1b[94m",
        purple: "\x1b[95m",
        erase: "\x1b[8m",
        bold: "\x1b[1m",
        italic: "\x1b[3m",
        underline: "\x1b[4m",
        endc: "\x1b[0m",
        inverse: "\x1b[7m",
        strikeout: "\x1b[9m",
    }
}

pub const ansi_constant: Ansi = get_ansi();

#[cfg(tests)]
mod tests {
    use super::_shift_left_asciified_text;
    use super::ansi_constant;


    #[test]
    fn test_asciify_str() {
        println!("'{}'", ansi_constant.black);
        assert_eq!(0, 0);
    }
}












