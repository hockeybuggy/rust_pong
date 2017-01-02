extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
// use graphics::types::Rectangle;

mod objects;
use objects::paddle::Paddle;
use objects::ball::Ball;


pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend
    ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
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

    fn update(&mut self, args: &UpdateArgs) {
        self.ball.update(
            &self.left_paddle,
            &self.right_paddle,
        );
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Rust Pong",
        [400, 400],
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let ball = Ball::new();

    let left_paddle = Paddle::new_left_paddle();
    let right_paddle = Paddle::new_right_paddle();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        ball: ball,
        left_paddle: left_paddle,
        right_paddle: right_paddle,
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::W) => game.left_paddle.move_up(),
                Button::Keyboard(Key::S) => game.left_paddle.move_down(),

                Button::Keyboard(Key::Up) => game.right_paddle.move_up(),
                Button::Keyboard(Key::Down) => game.right_paddle.move_down(),
                _ => println!("Other button"),
            }
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
