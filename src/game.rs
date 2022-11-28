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
    alive: bool,
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
            alive: true,
        })
    }

    pub fn step_game(&mut self) {
        self.step_snake();
        self.check_fruit();
        self.check_self_hit();
    }

    fn check_self_hit(&mut self) {
        let head = self.get_snake_head();

        for snake_body_point in self.get_snake_body() {
            if head == snake_body_point && !std::ptr::eq(head, snake_body_point) {
                self.alive = false;
                break;
            }
        }
    }

    fn check_fruit(&mut self) {
        let head = self.get_snake_head();

        if self.fruit.location != *head {
            self.snake.pop_back();
        } else {
            self.fruit = Game::generate_fruit(self.size.x, self.size.y);
        }
    }

    fn get_snake_head(&self) -> &Point {
        self.get_snake_body().front().expect("Snake is empty")
    }

    fn step_snake(&mut self) {
        use Direction::{Down, Left, Right, Up};

        let head = self.get_snake_head();

        let (x, y) = (head.x, head.y);
        let new_head = match self.direction {
            Up => Point {
                x,
                y: dec_loop_around(y, self.size.y),
            },
            Down => Point {
                x,
                y: inc_loop_around(y, self.size.y),
            },
            Left => Point {
                x: dec_loop_around(x, self.size.x),
                y,
            },
            Right => Point {
                x: inc_loop_around(x, self.size.x),
                y,
            },
        };

        self.snake.push_front(new_head);
    }

    pub fn get_snake_body(&self) -> &Snake {
        &self.snake
    }

    pub fn get_fruit(&self) -> &Fruit {
        &self.fruit
    }

    pub fn is_alive(&self) -> bool {
        self.alive
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

fn inc_loop_around(val: u16, max: u16) -> u16 {
    if val + 1 > max {
        0
    } else {
        val + 1
    }
}

fn dec_loop_around(val: u16, loop_around: u16) -> u16 {
    if val == 0 {
        loop_around
    } else {
        val - 1
    }
}
