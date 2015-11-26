#[macro_use] extern crate glium;
#[macro_use] extern crate imgui;
extern crate image;
extern crate nalgebra;
extern crate time;

use std::io::Cursor;
use std::borrow::Cow;

use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use glium::{glutin, DisplayBuild, Surface};
use image::GenericImage;
use imgui::ImGui;
use imgui::glium_renderer::Renderer;
use nalgebra::Vec2;
use time::{SteadyTime, Duration};


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
    speed: f32,
    scroll_speed: f32,
    use_double: bool,
}

struct App {
    tunables: Tunables,
    scale: f32,
    position: Vec2<f32>,
    mouse_position: Vec2<f32>,
    mouse_down: bool,
    last_frame: Option<SteadyTime>,
    delta: Option<Duration>,
}

impl App {
    fn new() -> App {
        App {
            tunables: Tunables {
                escape_threshold: 4.0,
                max_iteration: 100,
                speed: 0.1,
                scroll_speed: 4.0,
                use_double: false,
            },
            scale: 1.0,
            position: Vec2::new(0.0, 0.0),
            mouse_position: Vec2::new(0.0, 0.0),
            mouse_down: false,
            last_frame: None,
            delta: None,
        }
    }

    fn frame(&mut self) {
        let now = SteadyTime::now();
        if let Some(t) = self.last_frame {
            self.delta = Some(now - t);
        }

        self.last_frame = Some(now)
    }

    fn frame_delta_seconds(&self) -> Option<f32> {
        self.delta.map(|d| d.num_nanoseconds().unwrap() as f32 / 1_000_000_000.0)
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

    fn handle_mouse_move(&mut self, x: f32, y: f32) {
        let new_position = Vec2::new(x, y);
        if self.mouse_down {
            let diff = new_position - self.mouse_position;
            self.position = self.position - diff * self.scale;
        }

        self.mouse_position = new_position;
    }

    fn handle_mouse_click(&mut self, clicked: bool) {
        self.mouse_down = clicked
    }

    fn handle_scroll(&mut self, amount: f32) {
        let point_at = self.position + self.mouse_position * self.scale;
        let x = 1.0 - 0.1f32.powf(self.tunables.scroll_speed);
        self.scale *= x.powf(amount);
        self.position = point_at - self.mouse_position * self.scale;
    }
}


fn mandelbrot() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_depth_buffer(24)
        .with_gl_profile(glutin::GlProfile::Core)
        .build_glium().unwrap();

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    let mut renderer = Renderer::init(&mut imgui, &display).unwrap();

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
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image : glium::texture::RawImage1d<u8> = glium::texture::RawImage1d {
        data: Cow::Owned(image.into_raw()),
        width: image_dimensions.0,
        format: glium::texture::ClientFormat::U8U8U8U8,
    };
    let texture = glium::texture::Texture1d::new(&display, image).unwrap();

    let fprogram = glium::Program::from_source(&display,
                                               &include_str!("./shaders/vertex.glsl"),
                                               &include_str!("./shaders/fragment.glsl"),
                                               None).unwrap();

    let dprogram = glium::Program::from_source(&display,
                                               &include_str!("./shaders/dvertex.glsl"),
                                               &include_str!("./shaders/dfragment.glsl"),
                                               None).ok();


    let mut app = App::new();
    loop {
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        let mut mouse_pos = None;
        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _code, Some(key_code)) => {
                    app.handle_key(key_code);
                }
                Event::MouseMoved(pos) => {
                    imgui.set_mouse_pos(pos.0 as f32, pos.1 as f32);
                    mouse_pos = Some(pos)
                }
                Event::MouseWheel(delta) => {
                    match delta {
                        glutin::MouseScrollDelta::LineDelta(_, y) |
                        glutin::MouseScrollDelta::PixelDelta(_, y) => {
                            app.handle_scroll(y)
                        }
                    }
                }
                Event::MouseInput(state, MouseButton::Left) => {
                    imgui.set_mouse_down(&[state == ElementState::Pressed,
                                           false, false, false, false]);
                    app.handle_mouse_click(state == ElementState::Pressed);
                }
                _ => ()
            }
        }
        app.frame();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {
            escape_threshold: app.tunables.escape_threshold,
            max_iteration: app.tunables.max_iteration as u32,
            position: app.position,
            scale: app.scale,
            tex: &texture,
        };

        let program = if app.tunables.use_double {
            if let Some(ref p) = dprogram {
                p
            } else {
                println!("Doubles are not supported");
                app.tunables.use_double = false;
                &fprogram
            }
        } else {
            &fprogram
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        let fps = format!("FPS: {}", imgui.get_frame_rate());
        let ifps = format!("ms per frame: {}", 1000.0 / imgui.get_frame_rate());

        let ui = imgui.frame(width, height, app.frame_delta_seconds().unwrap_or(1.0 / 60.0));
        if !ui.want_capture_mouse() {
            if let Some(pos) = mouse_pos {
                app.handle_mouse_move(2.0 * (pos.0 as f32 / width as f32) - 1.0,
                                      -(2.0 * (pos.1 as f32 / height as f32) - 1.0));
            }
        }
        ui.window()
            .name(im_str!("I'm a little fractal"))
            .movable(true)
            .size((500.0, 200.0), imgui::ImGuiSetCond_FirstUseEver)
            .build(|| {
                ui.text(im_str!("w, s, a, d, drag - movement\nj, k, wheel - scale."));
                ui.text(fps.into());
                ui.text(ifps.into());
                ui.separator();
                ui.slider_i32(im_str!("Number of iterations"),
                              &mut app.tunables.max_iteration, 0, 1000).build();
                ui.slider_f32(im_str!("Escape threshold"),
                              &mut app.tunables.escape_threshold, 0.0, 10.0).build();
                ui.slider_f32(im_str!("Scroll slowness"),
                              &mut app.tunables.scroll_speed, 0.05, 10.0).build();
                ui.checkbox(im_str!("Use doubles"),
                            &mut app.tunables.use_double);
            });
        renderer.render(&mut target, ui).unwrap();
        target.finish().unwrap();
    }
}


fn main() {
    mandelbrot();
}
