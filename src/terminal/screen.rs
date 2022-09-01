use termion::color::Color;
use termion::event::Key;
use termion::event::Event;
use termion::event::MouseEvent;
use termion::input::Events;
use termion::input::TermRead;
use termion::input::MouseTerminal;

use termion::cursor::{
    self,
    DetectCursorPos
};

use termion::event::*;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use std::borrow::BorrowMut;
use std::io::Read;
use std::io::Stdout;
use std::io::{
    stdin,
    stdout,
    Stdin,
    Write
};
use std::process::Command;

use pad::{
    PadStr,
    Alignment
};

type MouseStdout = MouseTerminal<RawTerminal<Stdout>>;

fn init_term() -> (Stdin, MouseStdout) {
    let mut stdin = stdin();
    let mut stdout =
        MouseTerminal::from(stdout().into_raw_mode().unwrap());
    (stdin, stdout)
}

fn init_screen(mut stdout: MouseStdout) {
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn end_screen(mut stdout: MouseStdout) {
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Show,
    )
    .unwrap();
    stdout.flush().unwrap();
}
use termion::screen::AlternateScreen;

pub struct Screen {
    stdin:  Stdin,
    screen: AlternateScreen<MouseStdout> /* you might need to do this
                                          * stdout: Rc<RefCell<MouseStdout>>,
                                          * because i cant use screen.events(&mut self) { and use screen.println() here mutable 2 times in a row } */
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

pub enum StringFormatting {
    Centered,
    Left,
    Right
}

impl Screen {
    pub fn new() -> Self {
        let (stdin, stdout) = init_term();
        let screen = AlternateScreen::from(stdout);
        Self {
            stdin,
            screen
        }
    }

    pub fn init_screen(&mut self) {
        write!(
            self.screen,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
            termion::cursor::Hide
        )
        .unwrap();
        self.screen.flush().unwrap();
    }

    pub fn end_screen(mut self) {
        write!(
            self.screen,
            "{}",
            // termion::clear::AfterCursor,
            termion::cursor::Show,
        )
        .unwrap();
        self.screen.flush().unwrap();
    }

    pub fn refresh(&mut self) {
        self.screen.flush().unwrap();
    }

    pub fn print(
        &mut self,
        text: &str,
        y: usize,
        x: usize
    ) -> &mut Self {
        write!(
            self.screen,
            "{}{}{}",
            // Clear the screen.
            termion::cursor::Goto(x as u16, y as u16),
            termion::clear::CurrentLine,
            text,
        )
        .unwrap();
        self
    }

    pub fn print_lines(
        &mut self,
        lines: &[&str],
        y: usize,
        x: usize
    ) -> &mut Self {
        for (index, line) in lines.iter().enumerate() {
            write!(
                self.screen,
                "{}{}{}",
                // Clear the screen.
                termion::cursor::Goto(x as u16, (y + index) as u16),
                termion::clear::CurrentLine,
                line,
            )
            .unwrap();
        }
        self
    }

    // ┌──────────┐
    // │ salutare │
    // ├──────────┤
    // │   asd    │
    // ├──────────┤
    // │   asd    │
    // └──────────┘

    pub fn draw_rectangle(
        &mut self,
        width: usize,
        text: &[&str],
        y: usize,
        x: usize,
        formatting: Option<StringFormatting>
    ) -> &mut Self {
        let mut rectangle_vec =
            vec![format!("┌{}┐\n", "─".repeat(width - 2))];
        for line in text {
            let line = if let Some(ref formatting) = formatting {
                match formatting {
                    StringFormatting::Centered => {
                        line.pad(width - 2, ' ', Alignment::Middle, true)
                    },
                    StringFormatting::Left => {
                        line.pad(width - 2, ' ', Alignment::Left, true)
                    },
                    StringFormatting::Right => {
                        line.pad(width - 2, ' ', Alignment::Right, true)
                    },
                }
            } else {
                line.to_string()
            };
            rectangle_vec.push(format!("│{}│", line,))
        }
        rectangle_vec
            .push(format!("└{}┘\n", "─".repeat(width as usize - 2)));

        let mut rv = Vec::<&str>::new();
        for item in &rectangle_vec {
            rv.push(item);
        }
        // let rv: Vec<&str> =
        //     rectangle_vec.into_iter().map(|s| s.as_str()).collect();

        self.print_lines(&rv[..], y, x);

        // for (index, line) in rectangle_vec.into_iter().enumerate() {
        //     self.print(&line, y + index, x);
        // }

        self
    }

    pub fn clear(&mut self) -> &mut Self {
        write!(self.screen, "{}", termion::clear::All).unwrap();
        self
    }

    pub fn events(&mut self) -> Events<&mut Stdin> {
        self.stdin.by_ref().events()
    }

    pub fn handle_keys_loop<F>(
        &mut self,
        mut handler: F
    ) where
        F: FnMut(&Event, &mut Screen) {
        let stdin = stdin();
        for e in stdin.events() {
            let e = e.unwrap();
            match e {
                Event::Key(Key::Ctrl('c')) => break,
                _ => {
                    handler(&e, self);
                }
            }
            // after_match(&e, self);
        }
    }
    // match e {
    //     Event::Key(Key::Char('a')) => {
    //         write!(
    //             stdout,
    //             "{}{}{}",
    //             // Clear the screen.
    //             // termion::clear::All,
    //             termion::cursor::Goto(1, 10),
    //             "its me mario",
    //             // Goto (1,1).
    //             // Hide the cursor.
    //             termion::cursor::Hide,
    //         )
    //         .unwrap();
    //         stdout.flush().unwrap();
    //     },
    //     Event::Key(Key::Char('b')) => {
    //         write!(
    //             stdout,
    //             "{}{}{}",
    //             // Clear the screen.
    //             // termion::clear::All,
    //             termion::cursor::Goto(1, 20),
    //             "its working",
    //             // Goto (1,1).
    //             // Hide the cursor.
    //             termion::cursor::Hide,
    //         )
    //         .unwrap();
    //         stdout.flush().unwrap();
    //     },
    //     Event::Key(Key::Char('c')) => {
    //         write!(
    //             stdout,
    //             "{}{}{}",
    //             // Clear the screen.
    //             termion::clear::All,
    //             termion::cursor::Goto(1, 1),
    //             // Goto (1,1).
    //             // Hide the cursor.
    //             termion::cursor::Hide,
    //         )
    //         .unwrap();
    //         stdout.flush().unwrap();
    //     },
    //     Event::Key(Key::Char(character)) => {
    //         println!("{}", character);
    //         break;
    //     },

    //     Event::Mouse(m) => {
    //         // println!("asd");
    //         match m {
    //             MouseEvent::Press(_, a, b)
    //             | MouseEvent::Release(a, b)
    //             | MouseEvent::Hold(a, b) => {
    //                 write!(
    //                     stdout,
    //                     "{}",
    //                     termion::cursor::Goto(a, b)
    //                 )
    //                 .unwrap();
    //                 // let (x, y) = stdout.cursor_pos().unwrap();
    //                 write!(stdout, "{}", "*")
    //                     // write!(
    //                     //     stdout,
    //                     //     "{}{}Cursor is at:
    //                     //         ({},{}){}",
    //                     //     termion::cursor::Goto(5, 5),
    //                     //     termion::clear::UntilNewline,
    //                     //     5,
    //                     //     5,
    //                     //     termion::cursor::Goto(a, b)
    //                     // )
    //                     .unwrap();
    //                 stdout.flush().unwrap();
    //             },
    //         }
    //     },
    //     _ => {},
    // }
}

pub enum TermClear {
    All(termion::clear::All),
    AfterCursor(termion::clear::AfterCursor),
    BeforeCursor(termion::clear::BeforeCursor),
    CurrentLine(termion::clear::CurrentLine),
    UntilNewlin(termion::clear::UntilNewline)
}

pub enum TermCursor {
    Hide(termion::cursor::Hide),
    Show(termion::cursor::Show)
}

pub struct ScreenPrintBuilder {
    refresh: Option<bool>,
    stdout:  Option<MouseStdout>,
    x:       Option<u16>,
    y:       Option<u16>,
    clear:   Option<TermClear>,
    cursor:  Option<TermCursor>
}

impl Default for ScreenPrintBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenPrintBuilder {
    pub fn new() -> Self {
        let (stdin, stdout) = init_term();
        Self {
            refresh: Some(false),
            stdout:  Some(stdout),
            x:       None,
            y:       None,
            clear:   None,
            cursor:  None
        }
    }

    pub fn x(
        mut self,
        x: u16
    ) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(
        mut self,
        y: u16
    ) -> Self {
        self.y = Some(y);
        self
    }

    pub fn clear_after_cursor(mut self) -> Self {
        self.clear =
            Some(TermClear::AfterCursor(termion::clear::AfterCursor));
        self
    }

    pub fn show_cursor(mut self) -> Self {
        self.cursor = Some(TermCursor::Show(termion::cursor::Show));
        self
    }

    pub fn hide_cursor(mut self) -> Self {
        self.cursor = Some(TermCursor::Hide(termion::cursor::Hide));
        self
    }

    pub fn refresh(mut self) -> Self {
        self.refresh = Some(true);
        self
    }

    pub fn print(
        &mut self,
        line: &str
    ) {
        let (x, y) = (self.x.unwrap(), self.y.unwrap());
        let mut stdout = self.stdout.as_deref_mut().unwrap();
        write!(
            stdout,
            "{}{}{}",
            // Clear the screen.
            termion::cursor::Goto(x, y),
            termion::clear::CurrentLine,
            line,
        )
        .unwrap();
        if self.refresh.unwrap() {
            stdout.flush().unwrap();
        }
    }
}

pub fn printr(text: &str) {
    print!("\x1b[2K{}\r", text);
    std::io::stdout().flush().expect("failed to flush stdout");
}
