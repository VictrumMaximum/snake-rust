use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crossterm::event::{read, Event, KeyCode};

use crate::game::Direction;

pub enum Message {
    Exit,
    Direction(Direction),
}

pub fn start_controller() -> Receiver<Message> {
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    thread::spawn(move || loop {
        let event = read().expect("failed reading the event");
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char(c) => match c {
                    'q' => {
                        tx.send(Message::Exit)
                            .expect("Failed to send keyevent on channel");

                        break;
                    }
                    _ => {}
                },
                KeyCode::Down => {
                    tx.send(Message::Direction(Direction::Down)).unwrap();
                }
                KeyCode::Up => {
                    tx.send(Message::Direction(Direction::Up)).unwrap();
                }
                KeyCode::Left => {
                    tx.send(Message::Direction(Direction::Left)).unwrap();
                }
                KeyCode::Right => {
                    tx.send(Message::Direction(Direction::Right)).unwrap();
                }
                _ => {}
            }
        }
    });

    rx
}
