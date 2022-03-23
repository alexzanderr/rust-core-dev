use ansi_term::Colour::{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    Fixed,
    RGB
};
use hex_color::HexColor;
use ansi_term::Style;

use ansi_hex_color::colored as hex_colored;

use std::thread::sleep;
use std::time::Duration;

use std::io::stdout;
use std::io::Write;

use std::thread;


// https://stackoverflow.com/questions/25649423/sending-trait-objects-between-threads-in-rust
pub struct SpinnerDotsThread<'a, F>
where
    F: Fn() -> () + Send + 'static, {
    frames:              [&'a str; 10],
    _function:           F,
    message:             String,
    termination_message: String,
}

impl<'a, F> SpinnerDotsThread<'a, F>
where
    F: Fn() -> () + Send + 'static,
{
    pub fn new(
        message: String,
        _function: F,
        termination_message: String,
    ) -> SpinnerDotsThread<'a, F> {
        SpinnerDotsThread {
            frames: ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            _function,
            message,
            termination_message,
        }
    }

    pub fn run_default(_function: F) {
        let yellow_package = Yellow.paint("📦");
        let green_successfully = Green.paint("successfully");
        let running_message = format!("{yellow_package} loading ... ");
        let termination_message =
            format!("{yellow_package} terminated {green_successfully}");
        let spinner = SpinnerDotsThread::new(
            running_message,
            _function,
            termination_message,
        );
        spinner.execute();
    }

    pub fn run_with_args(
        message: String,
        _function: F,
        termination_message: String,
    ) {
        let spinner = SpinnerDotsThread::new(
            message,
            _function,
            termination_message,
        );
        spinner.execute();
    }

    pub fn execute(self) {
        let handle = thread::spawn(move || {
            (self._function)();
        });

        // very nice color: #2666
        // let cyan: HexColor = "#999999".parse().unwrap();
        // let cyan = RGB(cyan.r, cyan.g, cyan.b);
        let mut _iter = 0;
        while handle.is_running() {
            let frame = self.frames[_iter % 10];
            let frame = Yellow.paint(frame);
            print!("{} {}\r", frame, self.message);
            stdout().flush().expect("some error message");
            sleep(Duration::from_millis(50));
            _iter += 1;
        }
        println!("{}", self.termination_message);
    }
}
