use std::io::{stdout, Error, Write};

use controller::{start_controller, Message};
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use drawer::start_drawer;
use game::Game;

mod controller;
mod drawer;
mod game;

fn main() -> Result<(), Error> {
    let mut out = stdout();

    init_screen(&out)?;

    'restart_loop: loop {
        // TODO: restore screen in case of panic

        let mut game = Game::new()?;

        let rx = start_controller();

        start_drawer(out.by_ref(), &mut game, &rx)?;

        if game.is_alive() {
            break 'restart_loop;
        } else {
            'user_input_loop: loop {
                if let Ok(msg) = rx.recv() {
                    match msg {
                        Message::Exit => break 'restart_loop,
                        Message::Restart => break 'user_input_loop,
                        _ => {}
                    }
                }
            }
        }
    }

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
