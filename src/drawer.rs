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

const RENDER_LOOP_SLEEP_MS: u64 = 400;

pub fn start_drawer(
    mut out: impl Write,
    game: &mut Game,
    rx: &Receiver<Message>,
) -> Result<(), Error> {
    'game_loop: loop {
        let start_time = Instant::now();

        game.step_game();
        clear_and_draw(out.by_ref(), &game)?;

        if !game.is_alive() {
            break 'game_loop;
        }

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
                        if game.set_direction(dir) {
                            break 'controller_loop;
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

const SNAKE_HEAD_CONTENT: &str = "🐲";
const SNAKE_HEAD_DEAD: &str = "😵";

const SNAKE_BODY_CONTENT: &str = "🐍";
const SNAKE_BODY_GROWING_CONTENT: &str = "🌟";
const SNAKE_BODY_DEAD_CONTENT: &str = "❌";

const FRUIT_1_CONTENT: &str = "🍊";
const FRUIT_2_CONTENT: &str = "🍓";
const FRUIT_3_CONTENT: &str = "🍌";

fn clear_and_draw(mut out: impl Write, game: &Game) -> Result<(), Error> {
    clear_screen(out.by_ref())?;

    let score_str = format!("score: {}", game.score);
    queue!(
        out,
        MoveTo(game.size.x - score_str.len() as u16, 0),
        PrintStyledContent(score_str.with(Color::Green))
    )?;

    let fruit = game.get_fruit();

    let snake = &game.snake;
    let snake_head = snake.get_head();
    let snake_body = snake.get_body();

    let snake_head_style = if game.is_alive() {
        SNAKE_HEAD_CONTENT
    } else {
        SNAKE_HEAD_DEAD
    };
    let snake_body_style = if game.is_alive() {
        if game.snake.is_growing() {
            SNAKE_BODY_GROWING_CONTENT
        } else {
            SNAKE_BODY_CONTENT
        }
    } else {
        SNAKE_BODY_DEAD_CONTENT
    };

    for snake_point in snake_body {
        queue!(
            out,
            MoveTo(snake_point.x, snake_point.y),
            Print(snake_body_style)
        )?;
    }

    queue!(
        out,
        MoveTo(snake_head.x, snake_head.y),
        Print(snake_head_style)
    )?;

    let fruit_content = match fruit.points {
        1 => FRUIT_1_CONTENT,
        2 => FRUIT_2_CONTENT,
        _ => FRUIT_3_CONTENT,
    };

    queue!(
        out,
        MoveTo(fruit.location.x, fruit.location.y),
        Print(fruit_content)
    )?;

    if !game.is_alive() {
        queue!(
            out,
            MoveTo(0, 0),
            PrintStyledContent("game over".with(Color::Red)),
            MoveTo(0, 1),
            PrintStyledContent("press 'q' to exit, press 'r' to restart".with(Color::Red)),
        )?;
    }

    out.flush()
}

fn clear_screen(mut out: impl Write) -> Result<(), Error> {
    execute!(out, Clear(crossterm::terminal::ClearType::All))
}
