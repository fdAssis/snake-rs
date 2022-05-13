mod colors;
mod draw;
mod game;
mod select;
mod physics;
mod snake;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use draw::blocks_in_pixels;
use game::Game;
use select::Select;
use piston_window::*;

const WINDOW_TITLE: &'static str = "Snake Rust";
const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut select = Select {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            select.render(&args);
        }

        if let Some(args) = e.update_args() {
            select.update(&args);
        }
    }
}
