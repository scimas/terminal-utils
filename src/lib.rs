//! # Terminal Utilities
//!
//! A library to work with terminal.

use std::io::Write;

/// Clear terminal screen and position cursor on the top left corner.
pub fn clearscreen() {
    print!("\x1b[2J\x1b[1;1H");
    std::io::stdout()
        .flush()
        .expect("Couldn't clear the screen");
}

/// Print `s` starting at `line` (vertical position) and `column` (horizontal
/// position). Indexing is zero based.
pub fn printat(s: &str, line: u32, column: u32) {
    print!("\x1b[{0};{1}H", line + 1, column + 1);
    println!("{}", s);
}
