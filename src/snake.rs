use piston::input::*;
use opengl_graphics::{ GlGraphics };
use std::collections::LinkedList;
use std::iter::FromIterator;
use palette::_SNAKE_PRIMARY as SNAKE_PRIMARY;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right, Left, Up, Down
}

#[derive(Clone)]
pub struct SnakePiece(u32, u32);

pub struct Snake {
    body: LinkedList<SnakePiece>,
    pub dir: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let body: LinkedList<SnakePiece> = LinkedList::from_iter((vec![
            SnakePiece(4, 4),
            SnakePiece(3, 4),
        ]).into_iter());

        Snake {
            body,
            dir: Direction::Right,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        use graphics;
        let snake_part = graphics::rectangle::Rectangle::new_round(SNAKE_PRIMARY, 2.0);

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|p| SnakePiece(p.0 * width, p.1 * width))
            .map(|p| {
                graphics::rectangle::square(
                    p.0 as f64,
                    p.1 as f64,
                    width as f64,
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            squares.into_iter()
                .for_each(|square| snake_part.draw(square, &c.draw_state, c.transform, gl))
        })
    }

    pub fn update(&mut self, dim: u32) -> bool {
        let mut new_head: SnakePiece = (*self.body.front().expect("Snake has no body")).clone();

        // Give the new head location based on direction
        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        // Check if snake went out of bounds
        if (self.dir == Direction::Up && new_head.1 == 0)
        || (self.dir == Direction::Left && new_head.0 == 0)
        || (self.dir == Direction::Down && new_head.1 == dim - 1)
        || (self.dir == Direction::Right && new_head.0 == dim - 1) {
            return false;
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();

        true
    }

    pub fn change_dir(&mut self, dir: Direction) {
        let last_dir = self.dir.clone();

        self.dir = match dir {
            Direction::Up if last_dir != Direction::Down => Direction::Up,
            Direction::Down if last_dir != Direction::Up => Direction::Down,
            Direction::Left if last_dir != Direction::Right => Direction::Left,
            Direction::Right if last_dir != Direction::Left => Direction::Right,
            _ => last_dir
        };
    }

    pub fn is_collide(&self, x: u32, y: u32) -> bool {
        self.body.iter().any(|p| x == p.0 && y == p.1)
    }
}
