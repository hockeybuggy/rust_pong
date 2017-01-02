
pub struct Paddle {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
}

impl Paddle {
    pub fn move_up(&mut self) {
        self.y -= 10.0
    }

    pub fn move_down(&mut self) {
        self.y += 10.0
    }

    pub fn collision(&self, x: f64, y: f64) -> bool {
        if (x >= self.x && x < (self.x + self.width) &&
            y >= self.y && y < (self.y + self.height)) {
            return true;
        }
        return false;
    }

    pub fn rectangle(&mut self) -> Rectangle {
        use graphics::rectangle;

        return rectangle::rectangle_by_corners(
            self.x,
            self.y,
            self.x + self.width,
            self.y + self.height,
        );
    }
}

