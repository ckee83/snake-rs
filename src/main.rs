//! Snake game
//!
//! Completed code for [video by @youcodethings](https://youtu.be/HCwMb0KslX8)
//!
//! Author: ckee


extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

#[derive(Clone)]
struct SnakePiece(u32, u32);

struct Snake {
    body: LinkedList<SnakePiece>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
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

    fn update(&mut self) {
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

pub struct Game {
    gl: GlGraphics,
    rotation: f64,
    snake: Snake
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;
        use graphics::Transformed;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(0.0, 0.0, 150.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // wipe screen
            graphics::clear(GREEN, gl);

            let transform = c.transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-75.0, -75.0);

            // draw the box
            graphics::rectangle(RED, square, transform, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second
        self.rotation += 4.0 * args.dt;
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create window to display the game
    let mut window: GlutinWindow = WindowSettings::new(
            "Snake Game",
            [640, 480]
        )
        .opengl(opengl)
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        snake: Snake {
            body: LinkedList::from_iter((vec![SnakePiece(4,4), SnakePiece(3,4)]).into_iter()),
            dir: Direction::Right
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }

    println!("Hello, world!");
}
