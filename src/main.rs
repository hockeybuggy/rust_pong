#[macro_use]
extern crate glium;
extern crate image;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod teapot;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);


fn read_shader(shader_type: &str) -> String {
    let filename = format!("src/shaders/{}.glsl", shader_type);
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("############################");
    println!("vertex shader:");
    println!("############################");
    println!("{}", contents);
    return contents;
}


fn main() {
    use glium::{DisplayBuild, Surface};
    // use std::io::Cursor;

    let vertex_shader_src: &str = &read_shader("vertex");
    let fragment_shader_src: &str = &read_shader("fragment");

    // let image = image::load(Cursor::new(&include_bytes!("textures/teak.png")[..]),
    //                         image::PNG).unwrap().to_rgba();
    // let image_dimensions = image.dimensions();
    // let gl_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
    //     image.into_raw(),
    //     image_dimensions
    // );

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    // let texture = glium::texture::Texture2d::new(&display, gl_image).unwrap();

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None
    ).unwrap();


    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES
    ).unwrap();


    loop {
        let uniforms = uniform! {
            matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.5, 1.0);

        target.draw(
            (&positions, &normals),
            &indices,
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();

        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
