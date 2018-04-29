extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{ GlGraphics };
use std::collections::LinkedList;
use std::iter::FromIterator;

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
        Snake {
            body: LinkedList::from_iter((vec![
                SnakePiece(4, 4),
                SnakePiece(3, 4),
            ]).into_iter()),
            dir: Direction::Right,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        const BLUE: [f32; 4] = [0.2, 0.2, 1.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|p| SnakePiece(p.0 * 20, p.1 * 20))
            .map(|p| {
                graphics::rectangle::square(
                    p.0 as f64,
                    p.1 as f64,
                    20f64
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(BLUE, square, transform, gl))
        })
    }

    pub fn update(&mut self) {
        let mut new_head: SnakePiece = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}
