use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::input::{ RenderArgs, UpdateArgs };

use pong::paddle::Paddle;
use pong::ball::Ball;


pub struct Game {
    gl: GlGraphics,
    ball: Ball,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
}


impl Game {
    pub fn new(opengl: OpenGL) -> Game {
        let ball = Ball::new();

        let left_paddle = Paddle::new_left_paddle();
        let right_paddle = Paddle::new_right_paddle();

        return Game {
            gl: GlGraphics::new(opengl),
            ball: ball,
            left_paddle: left_paddle,
            right_paddle: right_paddle,
        };
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let x = (args.width / 2) as f64;
        let y = (args.height / 2) as f64;

        let ball_rect = self.ball.rectangle();

        let left_rect = self.left_paddle.rectangle();
        let right_rect = self.right_paddle.rectangle();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y);

            rectangle(GREEN, ball_rect, transform, gl);

            rectangle(RED, left_rect, transform, gl);
            rectangle(BLUE, right_rect, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.ball.update(
            &self.left_paddle,
            &self.right_paddle,
        );
    }
}

