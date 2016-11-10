extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL, GlGraphics };

mod app;
mod board;
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
    let board = board::Board {
        panel_x: 0,
        panel_y: 0
    };

    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        board: board
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        // if let Some(u) = e.update_args() {
        //     app.update(&u);
        // }

        if let Some(p) = e.press_args() {
            app.press(&p);
        }
    }
}
