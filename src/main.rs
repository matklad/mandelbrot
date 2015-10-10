#[macro_use] extern crate glium;
extern crate image;
extern crate nalgebra as na;
extern crate num;
extern crate rustc_serialize;
extern crate time;

use std::io::Cursor;
use std::thread;
use std::io::Read;
use std::fs;

use glium::{glutin, DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use image::GenericImage;
use na::Vec2;
use rustc_serialize::json;


#[derive(Copy, Clone)]
struct Vertex {
    in_pos: Vec2<f32>,
}

implement_vertex!(Vertex, in_pos);

impl Vertex {
    fn new(x: f32, y: f32) -> Vertex{
        Vertex { in_pos: Vec2::new(x, y) }
    }
}

#[derive(RustcDecodable)]
struct Tunables {
    escape_threshold: f32,
    max_iteration: u32,
    ms_per_frame: u32,
    speed: f32,
}

struct App<'a> {
    tunables: Tunables,
    scale: f32,
    position: Vec2<f32>,
    display: &'a GlutinFacade,
    program: glium::Program,
}

type SimpleResult<T=()> = Result<T, Box<std::error::Error>>;

fn read_file(path: &str) -> SimpleResult<String> {
    let mut file = try!(fs::File::open(path));
    let mut data = String::new();
    try!(file.read_to_string(&mut data));
    Ok(data)
}

impl<'a> App<'a> {
    fn new(display: &GlutinFacade) -> App {
        let (vertex_shader_src, fragment_shader_src) = App::load_shaders().unwrap();
        App {
            tunables: Tunables {
                escape_threshold: 4.0,
                max_iteration: 100,
                ms_per_frame: 17,
                speed: 0.1,
            },
            scale: 1.0,
            position: Vec2::new(0.0, 0.0),
            display: display,
            program: glium::Program::from_source(
                display, &vertex_shader_src, &fragment_shader_src, None).unwrap(),
        }
    }

    fn load_shaders() -> SimpleResult<(String, String)> {
        let vertex = try!(read_file("./shaders/vertex.glsl"));
        let fragment = try!(read_file("./shaders/fragment.glsl"));
        Ok((vertex, fragment))
    }

    fn reload(&mut self) -> SimpleResult {
        let data = try!(read_file("./config.json"));
        let data = try!(json::decode(&data));
        self.tunables = data;

        let (vertex, fragment) = try!(App::load_shaders());
        let program = try!(glium::Program::from_source(
            self.display, &vertex, &fragment, None));
        self.program = program;
        Ok(())
    }

    fn handle_key(&mut self, code: glutin::VirtualKeyCode) {
        let speed = self.tunables.speed;
        match code {
            glutin::VirtualKeyCode::J => self.scale *= 1.0 + speed,
            glutin::VirtualKeyCode::K => self.scale *= 1.0 - speed,
            glutin::VirtualKeyCode::W => self.position.y += speed * self.scale,
            glutin::VirtualKeyCode::S => self.position.y -= speed * self.scale,
            glutin::VirtualKeyCode::A => self.position.x -= speed * self.scale,
            glutin::VirtualKeyCode::D => self.position.x += speed * self.scale,
            glutin::VirtualKeyCode::R => match self.reload() {
                Ok(_) => println!("Reloaded config"),
                Err(e) => {
                    println!("Failed to reaload config");
                    println!("{}", e.description());
                }
            },
            _ => ()
        }
    }
}


fn mandelbrot() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_depth_buffer(24)
        .build_glium().unwrap();
    let mut app = App::new(&display);

    let shape = vec![Vertex::new(-1.0, -1.0),
                     Vertex::new(-1.0,  1.0),
                     Vertex::new( 1.0,  1.0),
                     Vertex::new(-1.0, -1.0),
                     Vertex::new( 1.0, -1.0),
                     Vertex::new( 1.0,  1.0),
                     ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let image = image::load(Cursor::new(&include_bytes!("./texture.png")[..]),
                            image::PNG).unwrap();
    let pixels: Vec<_> = image.pixels().map(|(_, _, p)| p).collect();
    let texture = glium::texture::Texture1d::new(&display, pixels).unwrap();

    loop {
        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(state, _code, key_code) => {
                    if state == glutin::ElementState::Pressed {
                        if let Some(code) = key_code {
                            app.handle_key(code)
                        }
                    }
                }
                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            escape_threshold: app.tunables.escape_threshold,
            max_iteration: app.tunables.max_iteration,
            position: app.position,
            scale: app.scale,
            tex: &texture,
        };
        target.draw(&vertex_buffer, &indices, &app.program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
        thread::sleep_ms(app.tunables.ms_per_frame);
    }
}


fn main() {
    mandelbrot();
}
