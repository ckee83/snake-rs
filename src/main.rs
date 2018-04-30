extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod game;
mod snake;
mod food;
mod text_renderer;
mod palette;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use game::Game;

fn main() {
    let opengl = OpenGL::V3_2;
    // Assume a 24x24 game zone
    // This leaves 12 "squares" of space for a sidebar
    // Game zone needs a 1 square-width border
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const GAME_DIM: u32 = 24;
    const SQUARE_WIDTH: u32 = HEIGHT / GAME_DIM;

    // Create window to display the game
    let mut window: GlutinWindow = WindowSettings::new(
            "Snake Game",
            [WIDTH, HEIGHT]
        )
        .opengl(opengl)
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it
    let mut game = Game::new(GlGraphics::new(opengl), GAME_DIM, SQUARE_WIDTH);

    // Create an event pump
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

    println!("GAME OVER");
}
