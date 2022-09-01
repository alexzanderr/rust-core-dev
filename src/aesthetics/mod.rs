pub use ansi_term::Color::Red;
pub use ansi_term::Color::Yellow;
pub use ansi_term::Color::Black;

pub mod ansi;

mod macros;
pub use crate::red;

mod ascii;
use ascii::asciify;
