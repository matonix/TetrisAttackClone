pub struct Board {
    pub panel_x: u32,
    pub panel_y: u32
}

impl Board {

    pub fn left(&mut self) {
        self.panel_x -= 1;
    }

    pub fn right(&mut self) {
        self.panel_x += 1;
    }

    pub fn up(&mut self) {
        self.panel_y -= 1;
    }

    pub fn down(&mut self) {
        self.panel_y += 1;
    }
}
