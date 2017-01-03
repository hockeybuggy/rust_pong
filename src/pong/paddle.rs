use graphics::types::Rectangle;

use pong::utils::Bounds;

pub struct Paddle {
    x: f64,
    y: f64,
    height: f64,
    width: f64,

    pub score: i64,
}

impl Paddle {
    pub fn new_left_paddle() -> Paddle {
        return Paddle {
            x: -180.0,
            y: 10.0,
            height: 100.0,
            width: 10.0,

            score: 0,
        };
    }

    pub fn new_right_paddle() -> Paddle {
        return Paddle {
            x: 180.0,
            y: 10.0,
            height: 100.0,
            width: 10.0,

            score: 0,
        };
    }

    pub fn move_up(&mut self, bounds: &Bounds) {
        self.y -= 10.0;
        if self.top() < bounds.top {
            self.y += 10.0;
        }
    }

    pub fn move_down(&mut self, bounds: &Bounds) {
        self.y += 10.0;

        if self.bottom() > bounds.bottom {
            self.y -= 10.0;
        }
    }

    fn left(&self) -> f64 {
        return self.x - self.width / 2.0;
    }

    fn right(&self) -> f64 {
        return self.x + self.width / 2.0;
    }

    fn top(&self) -> f64 {
        return self.y - self.height / 2.0;
    }

    fn bottom(&self) -> f64 {
        return self.y + self.height / 2.0;
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
    }

    pub fn collision(&self, x: f64, y: f64) -> bool {
        // collision will ball
        return x >= self.left() && x < self.right() &&
               y >= self.top() && y < self.bottom();
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

