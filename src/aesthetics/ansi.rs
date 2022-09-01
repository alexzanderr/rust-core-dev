use hex_color::HexColor;
use ansi_term::Color::RGB;
use paste::paste;

pub fn hex_to_rgb(hex: &str) -> ansi_term::Color {
    let hex_color = hex.parse::<HexColor>().unwrap();
    let r = hex_color.r;
    let g = hex_color.g;
    let b = hex_color.b;
    RGB(r, g, b)
}

pub struct RGBColors {
    pub onedark_background: ansi_term::Color
}

impl RGBColors {
    pub fn new() -> Self {
        Self {
            onedark_background: hex_to_rgb("#282C34")
        }
    }
}

use ansi_term::Color;

fn match_color_by_name(name: &str) -> Color {
    match name {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "purple" => Color::Purple,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "black" => Color::Black,
        _ => Color::White
    }
}

macro_rules! impl_function_for_color {
    ($($color_name:ident, $color_type:expr)*) => ($(
        pub fn $color_name(content: &str) -> String {
            $color_type.paint(content).to_string()
        }

        paste! {
            // TODO add impl AsRef<str> for all
            pub fn [<$color_name _bold>](content: impl AsRef<str>) -> String {
                let content = content.as_ref();
                $color_type.bold().paint(content).to_string()
            }

            pub fn [<$color_name _underline>](content: &str) -> String {
                $color_type.underline().paint(content).to_string()
            }

            pub fn [<$color_name _italic>](content: &str) -> String {
                $color_type.italic().paint(content).to_string()
            }

            pub fn [<$color_name _bg>](content: &str) -> String {
                Color::Black.on($color_type).paint(content).to_string()
            }

            pub fn [<$color_name _on_bg>](content: &str, background: &str) -> String {
                $color_type.on(match_color_by_name(background)).paint(content).to_string()
            }

            pub fn [<$color_name _bold_italic>](content: &str) -> String {
                $color_type.bold().italic().paint(content).to_string()
            }

            pub fn [<$color_name _bold_underline>](content: &str) -> String {
                $color_type.bold().underline().paint(content).to_string()
            }

            pub fn [<$color_name _bold_italic_underline>](content: &str) -> String {
                $color_type.bold().italic().underline().paint(content).to_string()
            }
        }
    )*)
}

impl_function_for_color! {
    red, Color::Red
    green, Color::Green
    blue, Color::Blue
    yellow, Color::Yellow
    purple, Color::Purple
    cyan, Color::Cyan
    white, Color::White
}

/// use programmer must import these
/// use paste::paste;
/// use ansi_term::Color;
/// in their program
#[macro_export]
macro_rules! impl_fixed_color_function {
    ($color_number: expr) => {
        paste! {
            pub fn [<fixed_color_ $color_number>](content: &str) -> String {
                Color::Fixed($color_number).paint(content).to_string()
            }

        }
    };
}
