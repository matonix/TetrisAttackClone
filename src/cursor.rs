pub struct Cursor {
    x: u32,
    y: u32
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {x: 2, y: 18}
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.x < 6 - 2 {
            self.x += 1;
        }
    }

    pub fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.y < 20 - 1 {
            self.y += 1;
        }
    }
}
