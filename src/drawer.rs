use std::{
    io::{Error, Write},
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::Clear,
};

use crate::{controller::Message, game::Game};

const RENDER_LOOP_SLEEP_MS: u64 = 500;

pub fn start_drawer(
    mut out: impl Write,
    mut game: Game,
    rx: Receiver<Message>,
) -> Result<(), Error> {
    'game_loop: loop {
        let start_time = Instant::now();

        game.step_game();
        clear_and_draw(out.by_ref(), &game)?;

        'controller_loop: loop {
            let elapsed = start_time.elapsed().as_millis() as u64;
            if elapsed > RENDER_LOOP_SLEEP_MS {
                break 'controller_loop;
            }
            let time_to_sleep: u64 = RENDER_LOOP_SLEEP_MS - elapsed;

            if let Ok(msg) = rx.recv_timeout(Duration::from_millis(time_to_sleep)) {
                match msg {
                    Message::Exit => break 'game_loop,
                    Message::Direction(dir) => {
                        game.set_direction(dir);
                        break 'controller_loop;
                    }
                }
            }
        }
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
