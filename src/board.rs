pub struct Board {
    pub panel_x: u32,
    pub panel_y: u32
}

impl Board {

    pub fn left(&mut self) {
        if self.panel_x > 0 {
            self.panel_x -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.panel_x < 6 - 1 {
            self.panel_x += 1;
        }
    }

    pub fn up(&mut self) {
        if self.panel_y > 0 {
            self.panel_y -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.panel_y < 20 - 1 {
            self.panel_y += 1;
        }
    }
}
