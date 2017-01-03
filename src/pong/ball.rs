use graphics::types::Rectangle;
use rand::random;

use pong::paddle::Paddle;


pub struct Bounds {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

pub struct Ball {
    x: f64,
    y: f64,
    size: f64,

    velocity_x: f64,
    velocity_y: f64,

    bounds: Bounds,
}

impl Ball {
    pub fn new() -> Ball {
        let bounds = Bounds {
            top: -195.0,
            bottom: 195.0,
            left: -195.0,
            right: 195.0,
        };

        let ball = Ball {
            x: 0.0,
            y: 0.0,
            size: 10.0,

            velocity_x: 0.4,
            velocity_y: 0.7,

            bounds: bounds,
        };
        return ball;
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

    pub fn update(&mut self, left_paddle: &Paddle, right_paddle: &Paddle) {
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

        if self.y < self.bounds.top || self.y > self.bounds.bottom {
            self.velocity_y = -self.velocity_y;
        }
    }

    pub fn left_scores(&self) -> bool {
        return self.x > self.bounds.right;
    }

    pub fn right_scores(&self) -> bool {
        return self.x < self.bounds.left;
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
