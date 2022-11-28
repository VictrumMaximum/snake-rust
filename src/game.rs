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

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub struct Game {
    pub snake: Snake,
    direction: Direction,
    fruit: Fruit,
    size: Point,
    alive: bool,
}

pub struct Fruit {
    pub location: Point,
}

pub struct Snake {
    body: LinkedList<Point>,
}

impl Snake {
    pub fn get_full_body(&self) -> impl Iterator<Item = &Point> {
        self.body.iter()
    }

    pub fn get_body(&self) -> impl Iterator<Item = &Point> {
        let mut it = self.get_full_body();
        it.next();
        it
    }

    pub fn get_head(&self) -> &Point {
        self.body.front().expect("Snake is empty")
    }

    fn cut_tail(&mut self) {
        self.body.pop_back();
    }

    fn add_head(&mut self, head: Point) {
        self.body.push_front(head);
    }
}

impl Game {
    pub fn new() -> Result<Game, Error> {
        let (columns, rows) = size()?;
        let middle = Point {
            x: columns / 2,
            y: rows / 2,
        };

        Ok(Game {
            snake: Snake {
                body: LinkedList::from([middle]),
            },
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
        let snake = &self.snake;
        let head = snake.get_head();
        let body = snake.get_body();

        let mut intersect = false;
        for snake_body_point in body {
            if head == snake_body_point {
                intersect = true;
                break;
            }
        }
        self.set_alive(!intersect);
    }

    fn check_fruit(&mut self) {
        let head = self.snake.get_head();

        if self.fruit.location == *head {
            self.fruit = Game::generate_fruit(self.size.x, self.size.y);
        } else {
            self.snake.cut_tail();
        }
    }

    fn step_snake(&mut self) {
        use Direction::{Down, Left, Right, Up};

        let snake = &self.snake;
        let head = snake.get_head();

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

        self.snake.add_head(new_head);
    }

    pub fn get_fruit(&self) -> &Fruit {
        &self.fruit
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }

    pub fn generate_fruit(columns: u16, rows: u16) -> Fruit {
        let mut rng = rand::thread_rng();

        Fruit {
            location: Point {
                x: rng.gen_range(0..columns),
                y: rng.gen_range(0..rows),
            },
        }
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if self.direction.opposite() != dir {
            self.direction = dir;
        }
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
