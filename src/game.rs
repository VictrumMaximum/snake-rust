use std::{collections::LinkedList, io::Error};

use crossterm::terminal::size;
use rand::Rng;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point {
            x: Point::round_to_multiple_of_two(x),
            y,
        }
    }

    pub fn up(&self, loop_around: u16) -> Point {
        Point {
            x: self.x,
            y: dec_loop_around(self.y, loop_around, false),
        }
    }

    pub fn down(&self, loop_around: u16) -> Point {
        Point {
            x: self.x,
            y: inc_loop_around(self.y, loop_around, false),
        }
    }

    pub fn left(&self, loop_around: u16) -> Point {
        Point {
            x: dec_loop_around(self.x, loop_around, true),
            y: self.y,
        }
    }

    pub fn right(&self, loop_around: u16) -> Point {
        Point {
            x: inc_loop_around(self.x, loop_around, true),
            y: self.y,
        }
    }

    fn round_to_multiple_of_two(x: u16) -> u16 {
        if x % 2 == 0 {
            return x;
        }

        if x > 1 {
            return x - 1;
        }
        return x + 1;
    }
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

pub struct Fruit {
    pub location: Point,
    pub points: u16,
}

pub struct Snake {
    body: LinkedList<Point>,
    growing: u16,
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

    pub fn is_growing(&self) -> bool {
        self.growing > 0
    }
}

pub struct Game {
    pub snake: Snake,
    direction: Direction,
    fruit: Fruit,
    pub size: Point,
    alive: bool,
    pub score: u64,
}

impl Game {
    pub fn new() -> Result<Game, Error> {
        let (columns, rows) = size()?;
        let middle = Point::new(columns / 2, rows / 2);

        Ok(Game {
            snake: Snake {
                body: LinkedList::from([middle]),
                growing: 2,
            },
            direction: Direction::Right,
            fruit: Game::generate_fruit(columns, rows),
            size: Point {
                x: Point::round_to_multiple_of_two(columns),
                y: rows,
            },
            alive: true,
            score: 0,
        })
    }

    pub fn step_game(&mut self) {
        self.step_snake();
        self.check_self_hit();
        self.check_fruit();
        self.check_growing();
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
            self.snake.growing = self.fruit.points;
            self.score += self.fruit.points as u64;
        }
    }

    fn check_growing(&mut self) {
        if self.snake.growing > 0 {
            self.snake.growing -= 1;
        } else {
            self.snake.cut_tail();
        }
    }

    fn step_snake(&mut self) {
        use Direction::{Down, Left, Right, Up};

        let snake = &self.snake;
        let head = snake.get_head();

        let new_head = match self.direction {
            Up => head.up(self.size.y),
            Down => head.down(self.size.y),
            Left => head.left(self.size.x),
            Right => head.right(self.size.x),
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
            location: Point::new(rng.gen_range(0..columns), rng.gen_range(0..rows)),
            points: rng.gen_range(1..=3),
        }
    }

    pub fn set_direction(&mut self, dir: Direction) -> bool {
        if self.direction.opposite() != dir {
            self.direction = dir;
            return true;
        }
        return false;
    }
}

fn inc_loop_around(val: u16, max: u16, double_step: bool) -> u16 {
    let step = if double_step { 2 } else { 1 };

    if (val + step + 1) > max {
        0
    } else {
        val + step
    }
}

fn dec_loop_around(val: u16, loop_around: u16, double_step: bool) -> u16 {
    let step = if double_step { 2 } else { 1 };

    if (val as i16 - step as i16) < 0 {
        loop_around - step
    } else {
        val - step
    }
}
