#[macro_use] extern crate glium;
#[macro_use] extern crate imgui;
extern crate image;
extern crate nalgebra;
extern crate num;

use std::io::Cursor;
use std::thread;

use glium::{glutin, DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use imgui::ImGui;
use imgui::glium_renderer::Renderer;
use image::GenericImage;
use nalgebra::Vec2;


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
    escape_threshold: f32,
    max_iteration: i32,
    ms_per_frame: i32,
    speed: f32,
}

struct App {
    tunables: Tunables,
    scale: f32,
    position: Vec2<f32>,
    program: glium::Program,
}

impl App {
    fn new(display: &GlutinFacade) -> App {
        App {
            tunables: Tunables {
                escape_threshold: 4.0,
                max_iteration: 100,
                ms_per_frame: 17,
                speed: 0.1,
            },
            scale: 1.0,
            position: Vec2::new(0.0, 0.0),
            program: glium::Program::from_source(
                display,
                &include_str!("./shaders/vertex.glsl"),
                &include_str!("./shaders/fragment.glsl"),
                None).unwrap(),
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

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    let mut renderer = Renderer::init(&mut imgui, &display).unwrap();

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
                Event::Closed => return,
                Event::KeyboardInput(state, _code, key_code) => {
                    if state == ElementState::Pressed {
                        if let Some(code) = key_code {
                            app.handle_key(code)
                        }
                    }
                }
                Event::MouseMoved(pos) => imgui.set_mouse_pos(pos.0 as f32, pos.1 as f32),
                Event::MouseInput(state, MouseButton::Left) =>
                    imgui.set_mouse_down(&[state == ElementState::Pressed,
                                           false, false, false, false]),
                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            escape_threshold: app.tunables.escape_threshold,
            max_iteration: app.tunables.max_iteration as u32,
            position: app.position,
            scale: app.scale,
            tex: &texture,
        };
        target.draw(&vertex_buffer, &indices, &app.program, &uniforms,
                    &Default::default()).unwrap();

        let (width, height) = target.get_dimensions();
        let ui = imgui.frame(width, height, 0.0017);
        ui.window()
            .name(im_str!("Hello world"))
            .movable(true)
            .size((500.0, 160.0), imgui::ImGuiSetCond_FirstUseEver)
            .build(|| {
                ui.text(im_str!("w, s, a, d - movement\nj, k - scale."));
                ui.slider_i32(im_str!("Number of iterations"),
                              &mut app.tunables.max_iteration, 0, 1000).build();
                ui.slider_f32(im_str!("Escape threshold"),
                              &mut app.tunables.escape_threshold, 0.0, 10.0).build();
                ui.slider_i32(im_str!("ms per frame"),
                              &mut app.tunables.ms_per_frame, 1, 100).build();
            });
        renderer.render(&mut target, ui).unwrap();
        target.finish().unwrap();
        thread::sleep_ms(app.tunables.ms_per_frame as u32);
    }
}


fn main() {
    mandelbrot();
}
