extern crate opengl_graphics;

use opengl_graphics::{OpenGL};
use crate::app::{ExampleApp, AppBase};

mod app;

fn main() {
    let mut app = ExampleApp::new(OpenGL::V3_2);
    app.run();
}
