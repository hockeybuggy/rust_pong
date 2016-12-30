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

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Glium tutorial")
        .with_depth_buffer(24)
        .with_dimensions(600, 600)
        .build_glium()
        .unwrap();

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
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let light = [-1.0, 0.4, 0.9f32];

    let mut t: f32 = -0.5;
    let scale = 0.01;

    loop {
        t = t + 0.00005;
        if t >= 0.5 {
            t = -0.5;
        };

        let matrix = [
            [(scale / t).cos(), (scale / t).sin(), 0.0, 0.0],
            [-(scale / t).sin(), (scale / t).cos(), 0.0, 0.0],
            [0.0, 0.0, scale, 0.0],
            [0.0, 0.0, 3.0, 1.0f32],
        ];

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [ f * aspect_ratio, 0.0, 0.0, 0.0],
                [ 0.0, f, 0.0, 0.0],
                [ 0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
                [ 0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
            ]
        };

        let uniforms = uniform! {
            matrix: matrix,
            perspective: perspective,
            u_light: light,
        };

        target.draw(
            (&positions, &normals),
            &indices,
            &program,
            &uniforms,
            &params,
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
