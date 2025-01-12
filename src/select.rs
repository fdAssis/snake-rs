extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;


pub struct Select {
  pub gl: GlGraphics, // OpenGL drawing backend.
  pub rotation: f64,  // Rotation for the square.
}

impl Select {
  pub fn render(&mut self, args: &RenderArgs) {
      use graphics::*;

      const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
      const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

      let square = rectangle::square(50.0, 0.0, 50.0);
      let rotation = self.rotation;
      let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

      self.gl.draw(args.viewport(), |c, gl| {
          // Clear the screen.
          clear(GREEN, gl);

          let transform = c
              .transform
              .trans(x, y)
              .rot_rad(rotation)
              .trans(-25.0, -25.0);

          // Draw a box rotating around the middle of the screen.
          rectangle(RED, square, transform, gl);
      });
  }

  pub fn update(&mut self, args: &UpdateArgs) {
      // Rotate 2 radians per second.
      self.rotation += 2.0 * args.dt;
  }
}