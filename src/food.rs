use piston::input::*;
use opengl_graphics::{ GlGraphics };
use rand::Rng;
use rand::thread_rng;
use snake::Snake;
use palette::_FOOD_PRIMARY as FOOD_PRIMARY;

pub struct Food {
    pub dimension: u32,
    pub width: u32,
    pos_x: u32,
    pos_y: u32,
    rotation: f64,
}

impl Food {
    pub fn new(dimension: u32, width: u32, snake: &Snake) -> Food {
        // pick a random coord for new food and make sure our snake isn't there
        let mut r = thread_rng();
        let mut food = Food {
            dimension,
            width,
            pos_x: 0,
            pos_y: 0,
            rotation: 0.0,
        };

        loop {
            let x = r.gen_range(0, dimension - 2) + 1;
            let y = r.gen_range(0, dimension - 2) + 1;
            if !snake.is_collide(x, y) {
                food.pos_x = x;
                food.pos_y = y;
                break;
            }
        };

        println!("Rendering food at {:?}, {:?}", food.pos_x, food.pos_y);

        food
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        use graphics;
        use graphics::Transformed;
        let square = graphics::rectangle::Rectangle::new_round(FOOD_PRIMARY, 2.0);

        gl.draw(args.viewport(), |c, gl| {
            let rotation = self.rotation;
            let x_rot_axis: f64 = (self.pos_x as f64 * width as f64) + (0.5 * width as f64);
            let y_rot_axis: f64 = (self.pos_y as f64 * width as f64) + (0.5 * width as f64);
            let transform = c.transform
                .trans(x_rot_axis, y_rot_axis)
                .rot_rad(rotation)
                .trans(-0.5 * width as f64, -0.5 * width as f64);

            let coords = graphics::rectangle::square(
                0.0 as f64,
                0.0 as f64,
                width as f64,
            );
            square.draw(coords, &c.draw_state, transform, gl)
        });
    }

    // Return true if snake ate food this update
    pub fn update(&mut self, args: &UpdateArgs, snake: &Snake) -> bool {
        // Rotate 3 radians per second
        self.rotation += 3.0 * args.dt;

        snake.is_collide(self.pos_x, self.pos_y)
    }
}
