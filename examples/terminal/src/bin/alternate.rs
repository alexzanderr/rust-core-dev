use termion::screen::AlternateScreen;
use std::io::{
    Write,
    stdout
};
use core_dev::system::pause;

fn main() {
    {
        let mut screen = AlternateScreen::from(stdout());
        write!(screen, "Writing to alternate screen!").unwrap();
        screen.flush().unwrap();
        pause();
    }
    println!("Writing to main screen.");
}
