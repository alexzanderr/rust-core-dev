use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;

// https://github.com/timvisee/rust-clipboard-ext
pub fn copy_to_clipboard(text: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text.into()).unwrap();
}

pub fn get_clipboard_text() -> String {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap()
}
