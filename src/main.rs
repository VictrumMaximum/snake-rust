use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use drawer::start_drawer;
use game::init_game;

mod drawer;
mod game;

fn main() -> Result<(), Error> {
    let mut out = stdout();

    init_screen(&out)?;

    let game = init_game()?;

    start_drawer(out.by_ref(), game)?;

    restore_screen(out.by_ref())?;

    Ok(())
}

fn init_screen(mut out: impl Write) -> Result<(), Error> {
    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, DisableBlinking, Hide)
}

fn restore_screen(mut out: impl Write) -> Result<(), Error> {
    disable_raw_mode()?;
    execute!(out, LeaveAlternateScreen, EnableBlinking, Show)
}
