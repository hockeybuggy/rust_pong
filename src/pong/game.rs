use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::{clear, rectangle, Graphics, Context, Transformed};
use graphics::types::Rectangle;
use graphics::ellipse;


use pong::ball::Ball;
use pong::paddle::Paddle;
use pong::utils::Bounds;


const COLOR_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const COLOR_BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const COLOR_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const COLOR_RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];


pub struct Game {
    gl: GlGraphics,

    pub bounds: Bounds,
    ball: Ball,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
}


impl Game {
    pub fn new(opengl: OpenGL, bounds: Bounds) -> Game {
        let ball = Ball::new();

        let left_paddle = Paddle::new_left_paddle();
        let right_paddle = Paddle::new_right_paddle();

        let game = Game {
            gl: GlGraphics::new(opengl),

            bounds: bounds,
            ball: ball,
            left_paddle: left_paddle,
            right_paddle: right_paddle,
        };

        return game;
    }

    fn print_score(&self) {
        println!("left: {}, right: {}",
                 self.left_paddle.score,
                 self.right_paddle.score,
                 );
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let ball_ellipse = self.ball.ellipse();

        let left_paddle_rect = self.left_paddle.rectangle();
        let right_paddle_rect = self.right_paddle.rectangle();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(COLOR_BLACK, gl);
            draw_scene(c, gl, ball_ellipse, [left_paddle_rect, right_paddle_rect]);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.ball.update(
            &self.bounds,
            &self.left_paddle,
            &self.right_paddle,
        );

        if self.ball.left_scores(&self.bounds) {
            self.left_paddle.increase_score();
            println!("Left Scores!!");
            self.print_score();
            self.ball = Ball::new();
        }
        if self.ball.right_scores(&self.bounds) {
            self.right_paddle.increase_score();
            println!("Right Scores!!");
            self.print_score();
            self.ball = Ball::new();
        }

    }
}

fn draw_scene<G: Graphics>(c: Context, gl: &mut G, ball: [f64; 3], paddles: [Rectangle; 2]) {
    let window_size = c.viewport.unwrap().window_size;
    let x = (window_size[0] / 2) as f64;
    let y = (window_size[1] / 2) as f64;

    let transform = c.transform.trans(x, y);

    // Draw middle line
    rectangle(
        [0.25, 0.25, 0.25, 0.25],
        [-5.0, -200.0, 10.0, 400.0],
        transform,
        gl,
    );
    let ball_ellipse = ellipse::Ellipse::new(COLOR_GREEN);
    // Draw ball
    ball_ellipse.draw(
        ellipse::circle(ball[0], ball[1], ball[2]),
        &c.draw_state,
        transform,
        gl,
    );
    // Draw paddles
    rectangle(COLOR_RED, paddles[0], transform, gl);
    rectangle(COLOR_BLUE, paddles[1], transform, gl);
}
