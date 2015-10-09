#[macro_use]
extern crate glium;
extern crate time;
extern crate num;
extern crate nalgebra as na;

use glium::{DisplayBuild, Surface};
use na::Vec2;
use std::thread;
use glium::glutin;

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

struct Tunables {
    max_iteration: u32,
    ms_per_frame: u32,
    speed: f32,
}

struct App {
    tunables: Tunables,
    scale: f32,
    position: Vec2<f32>,
}

impl App {
    fn new() -> App {
        App {
            tunables: Tunables {
                max_iteration: 100,
                ms_per_frame: 17,
                speed: 0.1,
            },
            scale: 1.0,
            position: Vec2::new(0.0, 0.0),
        }
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
            _ => ()
        }
    }
}

fn mandelbrot() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_depth_buffer(24)
        .build_glium().unwrap();

    let shape = vec![Vertex::new(-1.0, -1.0),
                     Vertex::new(-1.0,  1.0),
                     Vertex::new( 1.0,  1.0),
                     Vertex::new(-1.0, -1.0),
                     Vertex::new( 1.0, -1.0),
                     Vertex::new( 1.0,  1.0),
                     ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = include_str!("./shaders/vertex.glsl");
    let fragment_shader_src = include_str!("./shaders/fragment.glsl");

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None).unwrap();

    let mut app = App::new();

    loop {
        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(_state, _code, key_code) => {
                    if let Some(code) = key_code {
                        app.handle_key(code)
                    }
                }
                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {
            scale: app.scale,
            position: app.position,
            max_iteration: app.tunables.max_iteration,
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
        thread::sleep_ms(app.tunables.ms_per_frame);
    }
}


fn main() {
    thread::spawn(|| mandelbrot()).join().unwrap();
}
