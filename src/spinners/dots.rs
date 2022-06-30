use ansi_term::Colour::{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    Fixed,
    RGB,
};
use hex_color::HexColor;
use ansi_term::Style;

use ansi_hex_color::colored as hex_colored;

use std::thread::sleep;
use std::time::Duration;

use std::io::stdout;
use std::io::Write;

use std::error;
use std::result;

use std::thread;
use std::fmt;

#[derive(Debug)]
pub struct SpinnerClosureError;

// https://learning-rust.github.io/docs/e7.custom_error_types.html
// https://www.philipdaniels.com/blog/2019/defining-rust-error-types/
impl error::Error for SpinnerClosureError {
    fn description(&self) -> &str {
        "something went wrong inside the closure"
    }
}


impl fmt::Display for SpinnerClosureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "something went wrong inside the closure")
    }
}


type SpinnerClosure<T> = Box<dyn FnOnce() -> T + Send + 'static>;
type SpinnerClosureResult<T> = result::Result<T, SpinnerClosureError>;

// https://stackoverflow.com/questions/25649423/sending-trait-objects-between-threads-in-rust
pub struct SpinnerDotsThread<'a, T>
where
    T: Send + 'static, {
    frames:              [&'a str; 10],
    _function:           SpinnerClosure<T>,
    message:             String,
    termination_message: String,
}

impl<'a, T> SpinnerDotsThread<'a, T>
where
    T: Send + 'static,
{
    pub fn new(
        message: String,
        _function: SpinnerClosure<T>,
        termination_message: String,
    ) -> SpinnerDotsThread<'a, T> {
        SpinnerDotsThread {
            frames: [
                "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏",
            ],
            _function,
            message,
            termination_message,
        }
    }

    pub fn run_default(
        _function: SpinnerClosure<T>,
    ) -> SpinnerClosureResult<T> {
        let yellow_package = Yellow.paint("📦");
        let green_successfully = Green.paint("successfully");
        let running_message =
            format!("{yellow_package} loading ... ");
        let termination_message = format!(
            "{yellow_package} terminated {green_successfully}"
        );

        SpinnerDotsThread::run_with_args(
            running_message,
            _function,
            termination_message,
        )
    }

    pub fn run_with_args(
        message: String,
        _function: SpinnerClosure<T>,
        termination_message: String,
    ) -> SpinnerClosureResult<T> {
        let spinner = SpinnerDotsThread::new(
            message,
            _function,
            termination_message,
        );
        spinner.execute()
    }

    pub fn execute(self) -> SpinnerClosureResult<T> {
        let handle = thread::spawn(move || (self._function)());

        // very nice color: #2666
        // let cyan: HexColor = "#999999".parse().unwrap();
        // let cyan = RGB(cyan.r, cyan.g, cyan.b);
        let mut _iter = 0;
        while !handle.is_finished() {
            let frame = self.frames[_iter % 10];
            let frame = Yellow.paint(frame);
            print!("{} {}\r", frame, self.message);
            stdout().flush().expect("some error message");
            sleep(Duration::from_millis(50));
            _iter += 1;
        }
        println!("{}", self.termination_message);
        let result = handle.join();
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(SpinnerClosureError),
        }
    }
}
