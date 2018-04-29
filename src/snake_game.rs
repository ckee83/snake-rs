use piston::input::*;
use opengl_graphics::{ GlGraphics };
use snake::Snake;
use snake::Direction;
use food::Food;
use palette::_BLACK as BLACK;
use palette::_GAME_BG as GAME_BG;
use palette::_GAME_BORDER as GAME_BORDER;
use palette::_GAME_TEXT as GAME_TEXT;

pub struct Game {
    pub gl: GlGraphics,
    pub dimension: u32,
    pub width: u32,
    just_ate: bool,
    score: u64,
    snake: Snake,
    food: Food
}

impl Game {
    pub fn new(gl: GlGraphics, dimension: u32, width: u32) -> Game {
        let snake = Snake::new();
        let food = Food::new(dimension, width, &snake);
        Game {
            gl,
            dimension,
            width,
            just_ate: false,
            score: 0,
            snake,
            food,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics;
        use graphics::rectangle;

        let bg_offset: f64 = self.width.into();
        let bg_outer_width: f64 = self.dimension as f64 * bg_offset;
        let bg_width: f64 = bg_outer_width - ( 2.0 * bg_offset);
        let bg_outer_rect = rectangle::Rectangle::new_round(GAME_BORDER, 8.0);
        let bg_rect = rectangle::Rectangle::new_round(GAME_BG, 6.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // wipe screen
            graphics::clear(BLACK, gl);

            // draw game background
            bg_outer_rect.draw(
                [0.0, 0.0, bg_outer_width, bg_outer_width],
                &c.draw_state,
                c.transform,
                gl,
            );
            bg_rect.draw(
                [bg_offset, bg_offset, bg_width, bg_width],
                &c.draw_state,
                c.transform,
                gl,
            );
        });

        self.food.render(&mut self.gl, args, self.width);
        self.snake.render(&mut self.gl, args, self.width);
    }

    pub fn update(&mut self, args: &UpdateArgs) -> bool {
        if !self.snake.update(self.dimension, self.just_ate) { return false };

        if self.just_ate {
            self.score += 1;
            self.just_ate = false;
            println!("Score increased to {:?}", self.score);
        }

        self.just_ate = self.food.update(args, &self.snake);

        if self.just_ate {
            self.food = Food::new(self.dimension, self.width, &self.snake);
        }

        true
    }

    pub fn pressed(&mut self, btn: &Button) {
        // Here's an optional working approach to the `match btn` below
        // ```
        // let new_dir: Option<Direction> = match btn {
        //     &Button::Keyboard(Key::Up) => Some(Direction::Up),
        //     &Button::Keyboard(Key::Down) => Some(Direction::Down),
        //     &Button::Keyboard(Key::Left) => Some(Direction::Left),
        //     &Button::Keyboard(Key::Right) => Some(Direction::Right),
        //     _ => None,
        // };
        //
        // if let Some(d) = new_dir {
        //     self.snake.change_dir(d);
        // }
        // // The above could be replaced with:
        // // match new_dir {
        // //     Some(d) => self.snake.change_dir(d),
        // //     None => (),
        // //     // _ => (), also works here
        // // };
        // ```
        match btn {
            &Button::Keyboard(Key::Up) => self.snake.change_dir(Direction::Up),
            &Button::Keyboard(Key::Down) => self.snake.change_dir(Direction::Down),
            &Button::Keyboard(Key::Left) => self.snake.change_dir(Direction::Left),
            &Button::Keyboard(Key::Right) => self.snake.change_dir(Direction::Right),
            _ => (),
        };
    }
}
