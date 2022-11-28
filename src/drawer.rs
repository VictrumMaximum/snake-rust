use std::{
    io::{Error, Write},
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::{Color, Print, PrintStyledContent, Stylize},
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
        if !game.is_alive() {
            break 'game_loop;
        }
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

const SNAKE_CONTENT: &str = "*";
const FRUIT_CONTENT: &str = "+";

fn clear_and_draw(mut out: impl Write, game: &Game) -> Result<(), Error> {
    clear_screen(out.by_ref())?;

    let fruit = game.get_fruit();

    queue!(
        out,
        MoveTo(fruit.location.x, fruit.location.y),
        Print(FRUIT_CONTENT.with(Color::Yellow))
    )?;

    let snake = &game.snake;
    let snake_head = snake.get_head();
    let snake_body = snake.get_body();

    let snake_head_style = SNAKE_CONTENT.with(Color::Green);
    let snake_body_style = SNAKE_CONTENT.with(Color::White);

    queue!(
        out,
        MoveTo(snake_head.x, snake_head.y),
        PrintStyledContent(snake_head_style)
    )?;

    for snake_point in snake_body {
        queue!(
            out,
            MoveTo(snake_point.x, snake_point.y),
            PrintStyledContent(snake_body_style)
        )?;
    }

    out.flush()
}

fn clear_screen(mut out: impl Write) -> Result<(), Error> {
    execute!(out, Clear(crossterm::terminal::ClearType::All))
}
