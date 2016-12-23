#[macro_use]
extern crate glium;

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    loop {
        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    }
    // println!("Hello, world!");
}
