use piston::input::*;
use opengl_graphics::{ GlGraphics };
use snake::Snake;
use snake::Direction;
use food::Food;
use text_renderer::TextRenderer;
use palette::_BLACK as BLACK;
use palette::_T_BLACK as T_BLACK;
use palette::_GAME_BG as GAME_BG;
use palette::_GAME_BORDER as GAME_BORDER;

pub struct Game<'a> {
    pub gl: GlGraphics,
    pub dimension: u32,
    pub width: u32,
    just_ate: bool,
    score: u64,
    snake: Snake,
    food: Food,
    text_renderer: TextRenderer<'a>,
    paused: bool,
    game_over: bool,
}

impl<'a> Game<'a> {
    pub fn new(gl: GlGraphics, dimension: u32, width: u32) -> Game<'a> {
            let snake = Snake::new();
            let food = Food::new(dimension, width, &snake);
            let text_renderer = TextRenderer::new();

            Game {
                gl,
                dimension,
                width,
                just_ate: false,
                score: 0,
                snake,
                food,
                text_renderer,
                paused: false,
                game_over: false,
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

        // Draw the food and the snake
        self.food.render(&mut self.gl, args, self.width);
        self.snake.render(&mut self.gl, args, self.width);

        // Draw Scoreboard
        self.text_renderer.render(&mut self.gl, args);

        // If the game is paused, overlay the screen w/ "paused"
        if self.paused {
            self.render_overlay(args);
        }
    }

    pub fn render_overlay(&mut self, args: &RenderArgs) {
        use graphics::rectangle;

        // If the game is paused, overlay the screen w/ "paused"
        if !self.paused { return }

        // Create
        let dim: f64 = self.dimension as f64 * self.width as f64;
        let overlay = rectangle::Rectangle::new_round(T_BLACK, 0.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // overlay gameboard with semi-transparent black
            overlay.draw(
                [0.0, 0.0, dim, dim],
                &c.draw_state,
                c.transform,
                gl,
            );
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) -> bool {
        // Don't process updates if the game is paused
        if self.paused { return true; }

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
        // Disable controls while game is paused
        if self.paused {
            match btn {
                // allow unpause
                &Button::Keyboard(Key::Space) => self.paused = false,
                _ => (),
            };
        } else {
            match btn {
                // Spacebar pauses the game!
                &Button::Keyboard(Key::Space) => self.paused = !self.paused,
                // If pressed key was an arrow, change the snake's direction
                &Button::Keyboard(Key::Up) => self.snake.change_dir(Direction::Up),
                &Button::Keyboard(Key::Down) => self.snake.change_dir(Direction::Down),
                &Button::Keyboard(Key::Left) => self.snake.change_dir(Direction::Left),
                &Button::Keyboard(Key::Right) => self.snake.change_dir(Direction::Right),
                _ => (),
            };
        }

    }
}
