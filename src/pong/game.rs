use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::input::{ RenderArgs, UpdateArgs };

use pong::ball::Ball;
use pong::paddle::Paddle;
use pong::utils::Bounds;


pub struct Game {
    gl: GlGraphics,

    bounds: Bounds,
    ball: Ball,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
}


impl Game {
    pub fn new(opengl: OpenGL, bounds: Bounds) -> Game {
        let ball = Ball::new();

        let left_paddle = Paddle::new_left_paddle();
        let right_paddle = Paddle::new_right_paddle();

        println!("Starting game!");
        let game = Game {
            gl: GlGraphics::new(opengl),

            bounds: bounds,
            ball: ball,
            left_paddle: left_paddle,
            right_paddle: right_paddle,
        };
        game.print_score();
        return game;
    }

    fn print_score(&self) {
        println!("left: {}, right: {}",
                 self.left_paddle.score,
                 self.right_paddle.score,
                 );
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

            // Draw middle line
            rectangle([0.25, 0.25, 0.25, 0.25], [-5.0, -200.0, 10.0, 400.0], transform, gl);
            // Draw ball
            rectangle(GREEN, ball_rect, transform, gl);
            // Draw paddles
            rectangle(RED, left_rect, transform, gl);
            rectangle(BLUE, right_rect, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.ball.update(
            &self.bounds,
            &self.left_paddle,
            &self.right_paddle
        );

        if self.ball.left_scores(&self.bounds) {
            self.left_paddle.increase_score();
            println!("Left Scores!!");
            self.print_score();
            self.ball.reset();
        }
        if self.ball.right_scores(&self.bounds) {
            self.right_paddle.increase_score();
            println!("Right Scores!!");
            self.print_score();
            self.ball.reset();
        }

    }
}

