use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), Error> {
    let out = stdout();

    init_screen(&out)?;

    restore_screen(&out)?;

    Ok(())
}

fn init_screen(mut out: impl Write) -> Result<(), Error> {
    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, DisableBlinking)
}

fn restore_screen(mut out: impl Write) -> Result<(), Error> {
    disable_raw_mode()?;
    execute!(out, LeaveAlternateScreen, EnableBlinking)
}
