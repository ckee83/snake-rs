use piston::input::*;
use opengl_graphics::{ GlGraphics };
use snake::Snake;
use snake::Direction;
use food::Food;
use text_renderer::TextRenderer;
use palette::_BLACK as BLACK;
use palette::_T_BLACK as T_BLACK;
use palette::_SCORE_BG as _SCORE_BG;
use palette::_SCORE_BORDER as _SCORE_BORDER;
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

    fn reset(&mut self) {
        self.snake = Snake::new();
        self.food = Food::new(self.dimension, self.width, &self.snake);
        self.just_ate = false;
        self.score = 0;
        self.paused = false;
        self.game_over = false;
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics;
        use graphics::rectangle;

        let bg_offset: f64 = self.width.into();
        let bg_outer_width: f64 = self.dimension as f64 * bg_offset;
        let bg_width: f64 = bg_outer_width - ( 2.0 * bg_offset);
        let bg_sidepanel_rect = rectangle::Rectangle::new_round(_SCORE_BG, 6.0);
        let bg_sidepanel_outer_rect = rectangle::Rectangle::new_round(_SCORE_BORDER, 8.0);
        let bg_outer_rect = rectangle::Rectangle::new_round(GAME_BORDER, 8.0);
        let bg_rect = rectangle::Rectangle::new_round(GAME_BG, 6.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // wipe screen
            graphics::clear(BLACK, gl);

            // draw game background
            bg_sidepanel_outer_rect.draw(
                [bg_outer_width, 0.0, 640.0 - bg_outer_width, 480.0],
                &c.draw_state,
                c.transform,
                gl,
            );
            bg_sidepanel_rect.draw(
                [bg_outer_width + 9.0, 8.0, 639.0 - ( bg_outer_width + 16.0 ), 480.0 - 16.0],
                &c.draw_state,
                c.transform,
                gl,
            );
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

        // Draw the food
        self.food.render(&mut self.gl, args, self.width);
        // Draw the snake
        self.snake.render(&mut self.gl, args, self.width);
        // Draw Scoreboard
        let score_x = (self.width * self.dimension) + 14;
        let score_y = 0 + 40;
        let score_txt = format!("NOMS: {}", self.score);
        self.text_renderer.medium(&mut self.gl, args, score_x, score_y, &score_txt);

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


        let mut txt_x = (self.width * self.dimension) / 2 - 72;
        let txt_y = (self.width * self.dimension) / 2 - 8;
        let txt: String;

        if self.game_over {
            // render "Game Over" text
            txt = String::from("GAME OVER");
            txt_x -= 82;
            let restart_txt = String::from("[space] to go again");
            self.text_renderer.small(&mut self.gl, args, txt_x + 48, txt_y + 24, &restart_txt);
        } else {
            // render "Paused" text
            txt = String::from("Paused");
        }
        self.text_renderer.large(&mut self.gl, args, txt_x, txt_y, &txt);

    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Don't process updates if the game is paused
        if self.paused { return; }

        if !self.snake.update(self.dimension, self.just_ate) {
            self.paused = true;
            self.game_over = true;
            return;
        };

        if self.just_ate {
            self.score += 1;
            self.just_ate = false;
            println!("Score increased to {:?}", self.score);
        }

        self.just_ate = self.food.update(args, &self.snake);

        if self.just_ate {
            self.food = Food::new(self.dimension, self.width, &self.snake);
        };
    }

    pub fn pressed(&mut self, btn: &Button) {
        // Disable controls while game is paused
        if self.paused {
            match btn {
                // allow unpause
                &Button::Keyboard(Key::Space) => {
                    if self.game_over {
                        self.reset();
                    }
                    self.paused = false;
                },
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
