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
use std::error::Error;

pub trait AppBase {
    fn new(ver: OpenGL) -> Self;
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
    fn run(&mut self);
}

pub struct ExampleApp { gl: GlGraphics, ver: OpenGL, rot: f64, window: Window }

impl AppBase for ExampleApp {
    fn new(ver: OpenGL) -> Self {
        let win = WindowSettings::new("Example App", [200, 200])
            .graphics_api(ver)
            .exit_on_esc(true)
            .build();

        if win.is_err() {
            panic!("Couldn't create window: {}", win.err().unwrap())
        }

        return ExampleApp{
            gl: GlGraphics::new(ver),
            ver,
            rot: 0.0,
            window: win.unwrap(),
        }
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

    fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }
}