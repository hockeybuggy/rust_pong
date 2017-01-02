extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::types::Rectangle;


pub struct Paddle {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
}

impl Paddle {

    fn move_up(&mut self) {
        self.y -= 10.0
    }

    fn move_down(&mut self) {
        self.y += 10.0
    }

    fn rectangle(&mut self) -> Rectangle {
        use graphics::rectangle;

        return rectangle::rectangle_by_corners(
            self.x,
            self.y,
            self.x + self.width,
            self.y + self.height,
        );
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    left_paddle: Paddle,
    right_paddle: Paddle,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let x = (args.width / 2) as f64;
        let y = (args.height / 2) as f64;

        let left_rect = self.left_paddle.rectangle();
        let right_rect = self.right_paddle.rectangle();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y).trans(-25.0, -25.0);

            rectangle(RED, left_rect, transform, gl);
            rectangle(BLUE, right_rect, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second
        // self.right_paddle.y += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [400, 400],
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let left_paddle = Paddle {
        x: 180.0,
        y: 10.0,
        height: 100.0,
        width: 10.0,
    };

    let right_paddle = Paddle {
        x: -160.0,
        y: 10.0,
        height: 100.0,
        width: 10.0,
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        left_paddle: left_paddle,
        right_paddle: right_paddle,
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Up) => app.left_paddle.move_up(),
                Button::Keyboard(Key::Down) => app.left_paddle.move_down(),

                Button::Keyboard(Key::W) => app.right_paddle.move_up(),
                Button::Keyboard(Key::S) => app.right_paddle.move_down(),
                _ => println!("Other button"),
            }
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
