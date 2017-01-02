extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;


mod pong;
use pong::game::Game;


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


    let mut game = Game::new(opengl);

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
