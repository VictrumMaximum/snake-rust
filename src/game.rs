use std::{collections::LinkedList, io::Error};

use crossterm::terminal::size;
use rand::Rng;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    snake: Snake,
    direction: Direction,
    fruit: Fruit,
    size: Point,
}

pub struct Fruit {
    pub location: Point,
    points: u16,
}

type Snake = LinkedList<Point>;

impl Game {
    pub fn new() -> Result<Game, Error> {
        let (columns, rows) = size()?;
        let middle = Point {
            x: columns / 2,
            y: rows / 2,
        };

        Ok(Game {
            snake: LinkedList::from([middle]),
            direction: Direction::Right,
            fruit: Game::generate_fruit(columns, rows),
            size: Point {
                x: columns,
                y: rows,
            },
        })
    }

    pub fn step_game(&mut self) {
        self.step_snake();
    }

    fn step_snake(&mut self) {
        use Direction::{Down, Left, Right, Up};

        let head = self.snake.front().expect("Snake is empty");

        let (x, y) = (head.x, head.y);
        let new_head = match self.direction {
            Up => Point { x, y: y - 1 },
            Down => Point { x, y: y + 1 },
            Left => Point { x: x - 1, y },
            Right => Point { x: x + 1, y },
        };

        if self.fruit.location != new_head {
            self.snake.pop_back();
        } else {
            self.fruit = Game::generate_fruit(self.size.x, self.size.y);
        }

        self.snake.push_front(new_head);
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_fruit(&self) -> &Fruit {
        &self.fruit
    }

    pub fn generate_fruit(columns: u16, rows: u16) -> Fruit {
        let mut rng = rand::thread_rng();

        Fruit {
            location: Point {
                x: rng.gen_range(0..columns),
                y: rng.gen_range(0..rows),
            },
            points: 1,
        }
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }
}
