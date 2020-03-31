extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Event};
use piston::window::WindowSettings;
use graphics::math::transform_pos;

pub trait AppBase {
    fn new(gl: GlGraphics, ver: OpenGL) -> Self;
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
    fn run(&mut self, window: &mut Window);
}

pub struct ExampleApp { gl: GlGraphics, ver: OpenGL, rot: f64 }

impl AppBase for ExampleApp {
    fn new(gl: GlGraphics, ver: OpenGL) -> Self {
        return ExampleApp{ gl, ver, rot: 0.0 }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rot;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // rotate 2 radians / second
        self.rot += 2.0 * args.dt;
    }

    fn run(&mut self, window: &mut Window) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }
}