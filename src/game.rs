use std::io::Error;

use crossterm::terminal::size;
use rand::Rng;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    snake: Snake,
    direction: Direction,
    fruit: Fruit,
}

struct Fruit {
    location: Point,
    points: u16,
}

type Snake = Point;
// type Snake = Vec<Point>;

impl Game {
    pub fn step_game(&mut self) {
        self.step_snake();
    }

    fn step_snake(&mut self) {
        use Direction::{Down, Left, Right, Up};

        let (x, y) = (self.snake.x, self.snake.y);
        self.snake = match self.direction {
            Up => Point { x, y: y - 1 },
            Down => Point { x, y: y + 1 },
            Left => Point { x: x - 1, y },
            Right => Point { x: x + 1, y },
        }
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }
}

pub fn init_game() -> Result<Game, Error> {
    let (columns, rows) = size()?;
    let middle = Point {
        x: columns / 2,
        y: rows / 2,
    };

    let mut rng = rand::thread_rng();

    let fruit = Fruit {
        location: Point {
            x: rng.gen_range(0..columns),
            y: rng.gen_range(0..rows),
        },
        points: 1,
    };

    Ok(Game {
        // snake: vec![middle],
        snake: middle,
        direction: Direction::Right,
        fruit,
    })
}
