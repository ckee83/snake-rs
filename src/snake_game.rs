extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{ GlGraphics };
use snake::Snake;
use snake::Direction;

pub struct Game {
    pub gl: GlGraphics,
    rotation: f64,
    snake: Snake
}

impl Game {
    pub fn new(gl: GlGraphics) -> Game {
        Game {
            gl: gl,
            rotation: 0.0,
            snake: Snake::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
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

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second
        self.rotation += 4.0 * args.dt;
        self.snake.update();
    }

    pub fn pressed(&mut self, btn: &Button) {
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
