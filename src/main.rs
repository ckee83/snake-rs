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

use snake_game::Game;
pub mod snake_game;
pub mod snake;

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
    let mut game = Game::new(GlGraphics::new(opengl));

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
}
