use graphics::types::Rectangle;

use pong::utils::Bounds;

const ACCELERATION: f64 = 0.50;
const DECELERATION_FACTOR: f64 = 100.0;


pub struct Paddle {
    x: f64,
    y: f64,
    dy: f64,
    height: f64,
    width: f64,

    pub score: i64,
}

impl Paddle {
    pub fn new_left_paddle() -> Paddle {
        return Paddle {
            x: -180.0,
            y: 10.0,
            dy: 0.0,
            height: 100.0,
            width: 10.0,

            score: 0,
        };
    }

    pub fn new_right_paddle() -> Paddle {
        return Paddle {
            x: 180.0,
            y: 10.0,
            dy: 0.0,
            height: 100.0,
            width: 10.0,

            score: 0,
        };
    }

    pub fn move_up(&mut self) {
        self.dy = -ACCELERATION;
    }

    pub fn move_down(&mut self) {
        self.dy = ACCELERATION;
    }

    pub fn update(&mut self, bounds: &Bounds) {
        // Decelerate the paddles
        if self.dy > -0.01 && self.dy < 0.01 {
            self.dy = 0.0;  // Close enough
        }
        if self.dy > 0.0 {
            self.dy -= ACCELERATION / DECELERATION_FACTOR;
        }
        if self.dy < 0.0 {
            self.dy += ACCELERATION / DECELERATION_FACTOR;
        }

        // Move the paddles
        self.y += self.dy;

        // Do a bounds check to limit the paddle's movement
        if self.bottom() > bounds.bottom || self.top() < bounds.top {
            self.y -= self.dy;
            self.dy = 0.0;
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
        // Check for collision will ball represented by (x, y)
        return x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom();
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
