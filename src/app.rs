
use opengl_graphics::{ OpenGL, GlGraphics };
use board::Board;
use piston::input::*;
use graphics::*;

pub struct App {
    board: Board,
    bg_color: types::Color
}

impl App {
    pub fn new() -> Self {
        App {
            board: Board::new(),
            bg_color: [0.9, 0.9, 0.9, 1.0]
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        const NPANELS_X: u32 = 6;
        const NPANELS_Y: u32 = 20;

        let cx = args.width as f64 / 2.0;
        let cy = args.height as f64 / 2.0;
        let hw = args.width as f64 *0.9 / 2.0;
        let hh = args.height as f64 *0.9 / 2.0;
        let rect = [cx-hw, cy-hh, hw*2.0, hh*2.0];
        let ref c = Context::new_abs(args.width as f64, args.height as f64);
        gl.draw(args.viewport(), |_, gl| {
            clear(self.bg_color, gl);
            self.board.render(rect, c, gl);
        });
    }

    // pub fn update(&mut self, args: &UpdateArgs) {
    // }

    pub fn press(&mut self, args: &Button) {
        use piston::input::Button::Keyboard;
        if *args == Keyboard(Key::Left) {
            self.board.left();
        }

        if *args == Keyboard(Key::Right) {
            self.board.right();
        }

        if *args == Keyboard(Key::Up) {
            self.board.up();
        }

        if *args == Keyboard(Key::Down) {
            self.board.down();
        }
    }
}
