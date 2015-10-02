#[macro_use]
extern crate glium;
extern crate time;
extern crate num;
extern crate nalgebra as na;

use num::traits::One;

use glium::{DisplayBuild, Surface};
use na::{PerspMat3, Iso3, Pnt3, Vec2, Vec3, BaseFloat, Mat4, UnitQuat};



#[derive(Copy, Clone)]
struct Vertex {
    in_pos: Vec2<f32>,
}

implement_vertex!(Vertex, in_pos);




fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_depth_buffer(24)
        .build_glium().unwrap();

    let vertex1 = Vertex { in_pos: Vec2::new(0.0, 0.0) };
    let vertex2 = Vertex { in_pos: Vec2::new(1.0,  0.0) };
    let vertex3 = Vertex { in_pos: Vec2::new(1.0, 1.0) };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = include_str!("./shaders/vertex.glsl");

    let fragment_shader_src = include_str!("./shaders/fragment.glsl");

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None).unwrap();

    let proj = PerspMat3::<f32>::new(1.0, f32::pi() / 4.0, 0.1, 100.0);

    let view: Mat4<f32> = na::to_homogeneous(&{
        let mut transform = Iso3::one();
        transform.look_at_z(&Pnt3::new(0.0, 0.0, 8.0),
                            &Pnt3::new(0.0, 0.0, 0.0),
                            &Vec3::new(0.0, 1.0, 0.0));
        transform});

    let start = time::precise_time_ns();
    loop {
        let time_from_start = (time::precise_time_ns() - start) as f32 / 1000000.0;
        let rotation_angle = time_from_start * 0.001;

        let rotation_by_time = UnitQuat::new(Vec3::new(rotation_angle, 0.0, 0.0));
        let rotation_by_time = na::to_homogeneous(&rotation_by_time.to_rot());
        let mvp: na::Mat4<f32> = *proj.as_mat() * view * rotation_by_time;


        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {mvp: mvp};
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
