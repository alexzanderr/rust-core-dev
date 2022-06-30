#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    unreachable_code
)]

use core_dev::terminal::screen::Screen;
use core_dev::terminal::screen::ScreenPrintBuilder;
use core_dev::system::pause;

use std::panic;

use termion::event::Key;
use termion::event::Event;

use core_dev::terminal::screen::StringFormatting;


fn main() {
    color_backtrace::install();

    let mut screen = Screen::new();
    screen.init_screen();

    screen.draw_rectangle(
        20,
        &["asd", "asd"],
        1,
        1,
        Some(StringFormatting::Centered),
    );

    screen.draw_rectangle(
        20,
        &["asd", "asd"],
        10,
        1,
        Some(StringFormatting::Left),
    );

    screen.draw_rectangle(
        20,
        &["asd", "asd"],
        20,
        1,
        Some(StringFormatting::Right),
    );

    screen.refresh();
    pause();

    screen.end_screen();
    std::process::exit(1);

    // panic::set_hook(Box::new(|_| {
    //     println!("Custom panic hook");
    //     let mut stdout = std::io::stdout();
    //     use std::io::Write;
    //     write!(
    //         stdout,
    //         "{}",
    //         // termion::cursor::Goto(1, 1),
    //         // termion::clear::All,
    //         termion::cursor::Show,
    //     )
    //     .unwrap();
    //     stdout.flush().unwrap();
    // }));

    for i in 0..10 {
        screen.print(&format!("salutare {}", i), 1, 1);
        screen.print(&format!("salutare {}", i), 2, 1);
        screen.print(&format!("salutare {}", i), 3, 1);

        screen.refresh();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    let lines = vec!["salutare", "si", "bine", "te-am", "gasit"];
    screen.print_lines(&lines, 4, 1).refresh();

    let s = ScreenPrintBuilder::new()
        .y(10)
        .x(1)
        .clear_after_cursor()
        .hide_cursor()
        .refresh()
        .print("i just made builder pattern for curses screen");


    // tried to borrow as immutable
    screen.handle_keys_loop(|event, screen| match event {
        Event::Key(Key::Char('a')) => {
            screen.print("hello there", 15, 1).refresh();
        },
        Event::Key(Key::Char('b')) => {
            screen.print("hello there -a ", 15, 1).refresh();
        },
        _ => {},
    });


    screen.end_screen();
}
