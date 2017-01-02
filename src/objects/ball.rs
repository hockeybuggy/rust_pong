
pub struct Ball {
    x: f64,
    y: f64,
    size: f64,

    velocity_x: f64,
    velocity_y: f64,

    bounds: Bounds,
}

impl Ball {
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.velocity_x = 0.2;
        self.velocity_y = 0.3;
        // TODO set random speed
    }

    pub fn update(&mut self, left_paddle: &Paddle, right_paddle: &Paddle) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        if left_paddle.collision(self.x, self.y) || right_paddle.collision(self.x, self.y) {
            println!("BOUNCE");
            self.velocity_x = -self.velocity_x;
        }

        if self.y < self.bounds.top || self.y > self.bounds.bottom {
            self.velocity_y = -self.velocity_y;
        }

        if self.x < self.bounds.left || self.x > self.bounds.right {
            // TODO score a point for a side
            println!("RESET");
            self.reset();
        }
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
