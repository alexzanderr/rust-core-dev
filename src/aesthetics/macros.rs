#[macro_export]
macro_rules! red {
    [$argument: expr] => {{
        $crate::aesthetics::Red.paint($argument).to_string()
    }};

    [$($argument: expr), *] => {{
        let mut colored_string = String::new();
        $(
            colored_string.push_str(
                $crate::aesthetics::Red.paint($argument).to_string().as_str());
        )*
        colored_string
    }};
}

#[macro_export]
macro_rules! red_bold {
    [$argument: expr] => {{
        $crate::aesthetics::Red.bold().paint($argument).to_string()
    }};
    [$($argument: expr), *] => {{
        let mut colored_string = String::new();
        $(
            colored_string.push_str(
                $crate::aesthetics::Red.bold().paint($argument).to_string().as_str());
        )*
        colored_string
    }};
}

#[macro_export]
macro_rules! red_underline {
    [$argument: expr] => {{
        $crate::aesthetics::Red.underline().paint($argument).to_string()
    }};
    [$($argument: expr), *] => {{
        let mut colored_string = String::new();
        $(
            colored_string.push_str(
                $crate::aesthetics::Red.underline().paint($argument).to_string().as_str());
        )*
        colored_string
    }};
}

#[macro_export]
macro_rules! red_italic {
    [$argument: expr] => {{
        $crate::aesthetics::Red.italic().paint($argument).to_string()
    }};

    [$($argument: expr), *] => {{
        let mut colored_string = String::new();
        $(
            colored_string.push_str(
                $crate::aesthetics::Red.italic().paint($argument).to_string().as_str());
        )*
        colored_string
    }};
}

#[macro_export]
macro_rules! red_underline_italic {
    [$argument: expr] => {{
        $crate::aesthetics::Red.underline().italic().paint($argument).to_string()
    }};

    [$($argument: expr), *] => {{
        let mut colored_string = String::new();
        $(
            colored_string.push_str(
                $crate::aesthetics::Red.underline().italic().paint($argument).to_string().as_str());
        )*
        colored_string
    }};
}

#[macro_export]
macro_rules! yellow {
    [$argument: expr] => {{
        ansi_term::Color::Yellow.paint($argument).to_string()
    }};
    [$($argument: expr), *] => {{
        let mut colored_string = String::new();
        $(
            colored_string.push_str(
                $crate::aesthetics::Yellow.paint($argument).to_string().as_str());
        )*
        colored_string
    }};
}

mod logs {
    #[macro_export]
    macro_rules! warning {
        [] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            rgb_colors.onedark_background.on($crate::aesthetics::Yellow).paint("  warning  ").to_string()
        }};
        [$argument: expr] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            let mut colored_string = rgb_colors.onedark_background.on(
                $crate::aesthetics::Yellow).paint("  warning  ").to_string();
            colored_string.push_str(" ");
            colored_string.push_str($argument);
            colored_string
        }};
    }

    #[macro_export]
    macro_rules! WARNING {
        [] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            rgb_colors.onedark_background.on(
                $crate::aesthetics::Yellow).paint("  WARNING  ").to_string()
        }};
        [$argument: expr] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            let mut colored_string = rgb_colors.onedark_background.on(
                $crate::aesthetics::Yellow).paint("  WARNING  ").to_string();
            colored_string.push_str(" ");
            colored_string.push_str($argument);
            colored_string
        }};
    }

    #[macro_export]
    macro_rules! ERROR {
        [] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            rgb_colors.onedark_background.on(
                $crate::aesthetics::Red).paint("  ERROR  ").to_string()
        }};
        [$argument: expr] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            let mut colored_string = rgb_colors.onedark_background.on(
                $crate::aesthetics::Red).paint("  ERROR  ").to_string();
            colored_string.push_str(" ");
            colored_string.push_str($argument);
            colored_string
        }};
    }

    #[macro_export]
    macro_rules! error {
        [] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            rgb_colors.onedark_background.on(
                $crate::aesthetics::Red).paint("  error  ").to_string()
        }};
        [$argument: expr] => {{
            let rgb_colors = $crate::aesthetics::colors::RGBColors::new();
            let mut colored_string = rgb_colors.onedark_background.on(
                $crate::aesthetics::Red).paint("  error  ").to_string();
            colored_string.push_str(" ");
            colored_string.push_str($argument);
            colored_string
        }};
    }
}
