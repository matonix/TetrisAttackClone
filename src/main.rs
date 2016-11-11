extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL, GlGraphics };
use app::App;

mod app;
mod board;
mod cursor;
mod settings;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
	let size = (640, 480);
    let title = "tetris attack clone";
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(title, size)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut gl = GlGraphics::new(opengl);
    let mut app = App::new();
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {

        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        // if let Some(ref args) = e.update_args() {
        //     app.update(args);
        // }

        if let Some(ref args) = e.press_args() {
            app.press(args);
        }
    }
}
