extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::*;
use piston::window::WindowSettings;

mod pong;
use pong::game::Game;
use pong::utils::Bounds;


fn main() {
    let opengl = OpenGL::V3_2;

    let bounds = Bounds {
        top: -195.0,
        bottom: 195.0,
        left: -195.0,
        right: 195.0,
    };

    let mut window: Window = WindowSettings::new("Rust Pong", [400, 400])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut game = Game::new(opengl, bounds);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::W) => game.left_paddle.move_up(&game.bounds),
                Button::Keyboard(Key::S) => game.left_paddle.move_down(&game.bounds),

                Button::Keyboard(Key::Up) => game.right_paddle.move_up(&game.bounds),
                Button::Keyboard(Key::Down) => game.right_paddle.move_down(&game.bounds),
                _ => println!("Other button"),
            }
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
