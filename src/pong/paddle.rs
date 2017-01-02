use graphics::types::Rectangle;

pub struct Paddle {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
}

impl Paddle {
    pub fn new_left_paddle() -> Paddle {
        return Paddle {
            x: -180.0,
            y: 10.0,
            height: 100.0,
            width: 10.0,
        };
    }

    pub fn new_right_paddle() -> Paddle {
        return Paddle {
            x: 180.0,
            y: 10.0,
            height: 100.0,
            width: 10.0,
        };
    }

    pub fn move_up(&mut self) {
        self.y -= 10.0
    }

    pub fn move_down(&mut self) {
        self.y += 10.0
    }

    fn left(&self) -> f64 {
        return self.x - self.width / 2.0;
    }

    fn right(&self) -> f64 {
        return self.x + self.width / 2.0;
    }

    fn top(&self) -> f64 {
        return self.y + self.height / 2.0;
    }

    fn bottom(&self) -> f64 {
        return self.y - self.height / 2.0;
    }

    pub fn collision(&self, x: f64, y: f64) -> bool {
        return x >= self.left() && x < self.right() &&
               y >= self.bottom() && y < self.top();
    }

    pub fn rectangle(&mut self) -> Rectangle {
        use graphics::rectangle;

        return rectangle::rectangle_by_corners(
            self.left(),
            self.top(),
            self.right(),
            self.bottom(),
        );
    }
}

