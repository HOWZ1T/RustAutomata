extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use crate::app::{ExampleApp, AppBase};

mod app;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("spinning square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = ExampleApp::new(GlGraphics::new(opengl), OpenGL::V3_2);
    app.run(&mut window);
}
