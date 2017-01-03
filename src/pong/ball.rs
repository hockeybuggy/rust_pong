use graphics::types::Rectangle;
use rand::random;

use pong::paddle::Paddle;
use pong::utils::Bounds;


pub struct Ball {
    x: f64,
    y: f64,
    size: f64,

    velocity_x: f64,
    velocity_y: f64,
}

impl Ball {
    pub fn new() -> Ball {
        return Ball {
            x: 0.0,
            y: 0.0,
            size: 10.0,

            velocity_x: 0.4,
            velocity_y: 0.7,
        };
    }

    pub fn reset(&mut self) {
        // TODO Create new ball rather than reset. List of game balls.
        self.x = 0.0;
        self.y = 0.0;
        // TODO ensure delta(x) > delta(y)
        self.velocity_x = random::<f64>() - 0.5;
        self.velocity_y = random::<f64>() - 0.5;
        println!("{},{}", self.velocity_x, self.velocity_y);
    }

    pub fn update(&mut self, bounds: &Bounds, left_paddle: &Paddle, right_paddle: &Paddle) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        if left_paddle.collision(self.x, self.y) ||
           right_paddle.collision(self.x, self.y) {
            println!("BOUNCE");
            self.velocity_x = -self.velocity_x;
            self.velocity_y = if self.velocity_y.is_sign_positive() {
                self.velocity_y + 0.1
            } else {
                self.velocity_y - 0.1
            };
        }

        if self.y < bounds.top || self.y > bounds.bottom {
            self.velocity_y = -self.velocity_y;
        }
    }

    pub fn left_scores(&self, bounds: &Bounds) -> bool {
        return self.x > bounds.right;
    }

    pub fn right_scores(&self, bounds: &Bounds) -> bool {
        return self.x < bounds.left;
    }

    pub fn rectangle(&mut self) -> Rectangle {
        use graphics::rectangle;
        // Off set the ball so that the middle of the ball is it's position.
        let offset = self.size / 2.0;

        return rectangle::rectangle_by_corners(
            self.x - offset,
            self.y - offset,
            self.x + offset,
            self.y + offset,
        );
    }
}
