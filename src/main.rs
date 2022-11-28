use std::io::{stdout, Error, Write};

use controller::start_controller;
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use drawer::start_drawer;
use game::init_game;

mod controller;
mod drawer;
mod game;

fn main() -> Result<(), Error> {
    let mut out = stdout();

    init_screen(&out)?;

    // TODO: restore screen in case of panic

    let game = init_game()?;

    let rx = start_controller();

    start_drawer(out.by_ref(), game, rx)?;

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
