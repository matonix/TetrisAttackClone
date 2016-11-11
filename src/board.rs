use cursor::Cursor;
use graphics::*;
use opengl_graphics::GlGraphics;

pub struct Board {
    cursor: Cursor,
    board_color: types::Color
}

impl Board {
    pub fn new() -> Self {
        Board {
            cursor: Cursor::new(),
            board_color: [0.2, 0.2, 0.2, 0.5]
        }
    }

    pub fn render(&self, rect: types::Rectangle,
            c: &Context, gl: &mut GlGraphics) {
        Rectangle::new(self.board_color)
            .draw(
                rect,
                &DrawState::default(),
                c.transform,
                gl
            );
        // cursor.render(&args, &gl);
    }

    pub fn left(&mut self) {
        self.cursor.left();
    }

    pub fn right(&mut self) {
        self.cursor.right();
    }

    pub fn up(&mut self) {
        self.cursor.up();
    }

    pub fn down(&mut self) {
        self.cursor.down();
    }
}
