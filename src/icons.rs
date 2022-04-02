// use bevy_reflect::Reflect;
// use bincode::{config, Decode, Encode};


// not working
// #[derive(Encode, Decode, PartialEq, Debug)]
/// note that every field of this struct is unicode
/// even if they are named similary to ascii
pub struct IconsStruct<'a> {
    pub pipe:                  &'a str,
    pub up_left_corner:        &'a str,
    pub down_left_corner:      &'a str,
    pub hifen:                 &'a str,
    pub small_x:               &'a str,
    pub symlink:               &'a str,
    pub git:                   &'a str,
    pub python:                &'a str,
    pub snake:                 &'a str,
    pub document:              &'a str,
    pub dot:                   &'a str,
    pub right_arrow:           &'a str,
    pub check_mark:            &'a str,
    pub X:                     &'a str,
    pub x:                     &'a str,
    pub branch:                &'a str,
    pub lock_slim:             &'a str,
    pub please_cloud:          &'a str,
    pub hexagon:               &'a str,
    pub rocket:                &'a str,
    pub sattelite:             &'a str,
    pub wrench:                &'a str,
    pub manjaro:               &'a str,
    pub github:                &'a str,
    pub horizontal_bar:        &'a str,
    pub symbol:                &'a str,
    pub json:                  &'a str,
    pub windows:               &'a str,
    pub markdown:              &'a str,
    pub file:                  &'a str,
    pub folder:                &'a str,
    pub settings_wheel:        &'a str,
    pub empty_file:            &'a str,
    pub empty_file_full:       &'a str,
    pub docker:                &'a str,
    pub gnu:                   &'a str,
    pub rust:                  &'a str,
    pub trash:                 &'a str,
    pub react:                 &'a str,
    pub terminal:              &'a str,
    pub github_oval:           &'a str,
    pub ruby:                  &'a str,
    pub vim:                   &'a str,
    pub vscode:                &'a str,
    pub settings_folder:       &'a str,
    pub apple:                 &'a str,
    pub android:               &'a str,
    pub dragon:                &'a str,
    pub cocktail:              &'a str,
    pub lock:                  &'a str,
    pub html:                  &'a str,
    pub node_js:               &'a str,
    pub npm:                   &'a str,
    pub tail:                  &'a str,
    pub forbidden:             &'a str,
    pub love_letter:           &'a str,
    pub paint:                 &'a str,
    pub balena:                &'a str,
    pub dart:                  &'a str,
    pub crystal_globe:         &'a str,
    pub gear:                  &'a str,
    pub disc_icon:             &'a str,
    pub triangle:              &'a str,
    pub battery_full:          &'a str,
    pub battery_charging:      &'a str,
    pub battery_not_connected: &'a str,
    pub battery_half:          &'a str,
    pub battery_empty:         &'a str,
    pub please:                &'a str,
    pub fire:                  &'a str,
    pub thick_cross:           &'a str,
    pub rose:                  &'a str,
    pub clover:                &'a str,
    pub paseluta:              &'a str,
    pub orchideea:             &'a str,
    pub corn:                  &'a str,
    pub somethings:            &'a str,
    pub bigger_x:              &'a str,
    pub celebration:           &'a str,
    pub block:                 &'a str,
    pub linux:                 &'a str,
    pub bomb:                  &'a str,
    pub boom:                  &'a str,
    pub double_arrows:         &'a str,
    pub stars:                 &'a str,
    pub small_arrow:           &'a str,
    pub big_arrow:             &'a str,
    pub package:               &'a str,
    pub tools_:                &'a str,
    pub chain:                 &'a str,
    pub raising_hand:          &'a str,
    pub euro:                  &'a str,
    pub umbrella:              &'a str,
    pub cool:                  &'a str,
    pub thunder:               &'a str,
    pub lambda:                &'a str,
    pub cross:                 &'a str,
}


const fn initialize_icons_struct() -> IconsStruct<'static> {
    IconsStruct {
        pipe:                  "│",
        up_left_corner:        "╭─",
        down_left_corner:      "╰─",
        hifen:                 "─",
        small_x:               "×",
        symlink:               "",
        git:                   "",
        python:                "",
        snake:                 "🐍",
        document:              "📃",
        dot:                   "•",
        right_arrow:           "→",
        check_mark:            "✔️",
        X:                     "✘",
        x:                     "×",
        branch:                "╰─",
        lock_slim:             "🔒",
        please_cloud:          "🙏",
        hexagon:               "⬢",
        rocket:                "🚀",
        sattelite:             "🛰",
        wrench:                "🔧",
        manjaro:               "",
        github:                "",
        horizontal_bar:        "▬",
        symbol:                "ஜ",
        json:                  "",
        windows:               "",
        markdown:              "",
        file:                  "",
        folder:                "",
        settings_wheel:        "",
        empty_file:            "",
        empty_file_full:       "",
        docker:                "",
        gnu:                   "",
        rust:                  "",
        trash:                 "",
        react:                 "",
        terminal:              "",
        github_oval:           "",
        ruby:                  "",
        vim:                   "",
        vscode:                "",
        settings_folder:       "",
        apple:                 "",
        android:               "",
        dragon:                "",
        cocktail:              "",
        lock:                  "",
        html:                  "",
        node_js:               "",
        npm:                   "",
        tail:                  "",
        forbidden:             "🚫",
        love_letter:           "💌",
        paint:                 "🎨",
        balena:                "🐳",
        dart:                  "🎯",
        crystal_globe:         "🔮",
        gear:                  "⚙️",
        disc_icon:             "🔘",
        triangle:              "△",
        battery_full:          "",
        battery_charging:      "",
        battery_not_connected: "",
        battery_half:          "",
        battery_empty:         "",
        please:                "🙏",
        fire:                  "🔥",
        thick_cross:           "➕",
        rose:                  "🌹",
        clover:                "🍀",
        paseluta:              "🌸",
        orchideea:             "🌺",
        corn:                  "🌿",
        somethings:            "☄ | ☃ ",
        bigger_x:              "❌",
        celebration:           "🎉",
        block:                 "█",
        linux:                 " ",
        bomb:                  "💣",
        boom:                  "💥",
        double_arrows:         "»",
        stars:                 "✨",
        small_arrow:           "›",
        big_arrow:             "❯",
        package:               "📦",
        tools_:                "🛠 ",
        chain:                 "🔗",
        raising_hand:          "👋",
        euro:                  "€",
        umbrella:              "🌂",
        cool:                  "ℤ",
        thunder:               "⚡",
        lambda:                "λ",
        cross:                 "┼",
    }
}

pub const Icons: IconsStruct = initialize_icons_struct();
