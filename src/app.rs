use opengl_graphics::{ GlGraphics };
use board::Board;
use piston::input::*;


pub struct App {
    pub gl: GlGraphics,
    pub board: Board
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const NPANELS_X: u32 = 6;
        const NPANELS_Y: u32 = 20;

        const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
        const GRAY2: [f32; 4] = [0.5, 0.5, 0.5, 0.8];
        const RED:  [f32; 4] = [0.9, 0.1, 0.1, 1.0];

        let panel_size = (args.height / NPANELS_Y) as f64;
        let board_size_x = panel_size * NPANELS_X as f64;
        let board_size_y = panel_size * NPANELS_Y as f64;
        let x = panel_size * self.board.panel_x as f64;
        let y = panel_size * self.board.panel_y as f64;
        let board = rectangle::centered([0.0, 0.0, board_size_x, board_size_y]);
        let square = rectangle::square(x, y, panel_size);
        let square2 = rectangle::square(0.0, y + panel_size, panel_size);

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(1.0,1.0);
            clear(GRAY, gl);
            rectangle(GRAY2, board, transform, gl);
            rectangle(RED, square, transform, gl);
            rectangle(RED, square2, transform, gl);
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
