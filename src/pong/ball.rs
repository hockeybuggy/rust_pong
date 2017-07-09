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
        let rand1 = random::<f64>() - 0.5;
        let rand2 = random::<f64>() - 0.5;
        let abs_max = rand1.abs().max(rand2.abs()) + 0.1;
        let abs_min = rand1.abs().max(rand2.abs()) - 0.2;

        let velocity_x = if rand1.is_sign_positive() {
            abs_max
        } else {
            -abs_max
        };
        let velocity_y = if rand2.is_sign_positive() {
            abs_min
        } else {
            -abs_min
        };
        println!("{} {}", velocity_x, velocity_y);

        return Ball {
            x: 0.0,
            y: 0.0,
            size: 10.0,

            velocity_x: velocity_x,
            velocity_y: velocity_y,
        };
    }

    pub fn update(&mut self, bounds: &Bounds, left_paddle: &Paddle, right_paddle: &Paddle) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        if left_paddle.collision(self.x, self.y) || right_paddle.collision(self.x, self.y) {
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

    pub fn ellipse(&mut self) -> [f64; 3] {
        let radius = self.size / 2.0;
        return [self.x, self.y, radius];
    }
}
