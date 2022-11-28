use std::{
    io::{Error, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::Clear,
};

use crate::game::Game;

pub fn start_drawer(mut out: impl Write, mut game: Game) -> Result<(), Error> {
    let mut i = 0;
    let max_loop = 10;

    loop {
        game.step_game();
        clear_and_draw(out.by_ref(), &game)?;

        i += 1;
        if i > max_loop {
            break;
        }
        sleep(Duration::from_millis(500));
    }

    Ok(())
}

const SNAKE_CONTENT: &str = "+";

fn clear_and_draw(mut out: impl Write, game: &Game) -> Result<(), Error> {
    clear_screen(out.by_ref())?;
    let snake = game.get_snake();

    let styled_snake = SNAKE_CONTENT.with(Color::Green);
    execute!(
        out,
        MoveTo(snake.x, snake.y),
        PrintStyledContent(styled_snake)
    )
}

fn clear_screen(mut out: impl Write) -> Result<(), Error> {
    execute!(out, Clear(crossterm::terminal::ClearType::All))
}
