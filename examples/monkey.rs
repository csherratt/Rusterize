#![feature(core, path)]

extern crate gfx;
extern crate glfw;
extern crate rusterize;
extern crate genmesh;
extern crate image;
extern crate obj;
extern crate cgmath;
extern crate time;

use gfx::Device;
use glfw::Context;
use genmesh::{Triangulate, MapToVertices};
use genmesh::generators::Cube;
use rusterize::{Frame, Fragment};
use image::Rgb;
use cgmath::*;
use time::precise_time_s;
use std::num::Float;

const SIZE: u32 = 1024;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
                        .ok().expect("failed to init glfw");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(SIZE, SIZE, "SW raster example.", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    window.set_key_polling(true);

    let device = gfx::GlDevice::new(|s| window.get_proc_address(s));
    let mut graphics = gfx::Graphics::new(device);

    let texture_info = gfx::tex::TextureInfo {
        width: SIZE as u16, height: SIZE as u16, depth: 1, levels: 1,
        kind: gfx::tex::TextureKind::Texture2D,
        format: gfx::tex::Format::Unsigned(gfx::tex::Components::RGB, 8, gfx::attrib::IntSubType::Normalized)
    };
    let image_info = texture_info.to_image_info();

    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let light_normal = Vector4::new(10., 10., 10., 0.).normalize();
    let kd = Vector4::new(64., 128., 64., 1.);
    let ka = Vector4::new(16., 16., 16., 1.);

    let proj = cgmath::perspective(cgmath::deg(60.0f32), 1.0, 0.01, 100.0);
    let mut frame = Frame::new(SIZE, SIZE);

    let texture = graphics.device.create_texture(texture_info).unwrap();
    graphics.device.update_texture(&texture, &image_info, frame.frame.as_slice()).unwrap();

    let mut texture_frame = gfx::Frame::new(SIZE as u16, SIZE as u16);
    texture_frame.colors.push(gfx::Plane::Texture(texture, 0, None));

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    window.set_should_close(true),
                _ => {},
            }
        }

        let time = precise_time_s() as f32;
        let cam_pos = {
            // Slowly circle the center
            let x = (0.25*time).sin();
            let y = (0.25*time).cos();
            Point3::new(x * 2.0, y * 2.0, 2.0)
        };
        let view: AffineMatrix3<f32> = Transform::look_at(
            &cam_pos,
            &Point3::new(0.0, 0.0, 0.0),
            &Vector3::unit_y(),
        );

        let mat = proj.mul_m(&view.mat);

        let vertex = monkey.indices().iter().map(|x| *x)
                           .vertex(|(p, _, n)| { (obj.position()[p], obj.normal()[n.unwrap()]) })
                           .vertex(|(p, n)| (mat.mul_v(&Vector4::new(p[0], p[1], p[2], 1.)).into_fixed(), n))
                           .triangulate();

        struct V {
            ka: Vector4<f32>,
            kd: Vector4<f32>,
            light_normal: Vector4<f32>
        }

        impl Fragment<([f32; 4], [f32; 3])> for V {
            type Color = Rgb<u8>;

            fn fragment(&self, (_, n) : ([f32; 4], [f32; 3])) -> Rgb<u8> {
                let normal = Vector4::new(n[0], n[1], n[2], 0.);
                let v = self.kd.mul_s(self.light_normal.dot(&normal).partial_max(0.)) + self.ka;
                Rgb([v.x as u8, v.y as u8, v.z as u8])
            }
        }

        frame.clear();
        frame.raster(vertex, V{ka: ka, kd: kd, light_normal: light_normal});
        graphics.device.update_texture(&texture, &image_info, frame.frame.as_slice()).unwrap();

        graphics.renderer.blit(&texture_frame,
            gfx::Rect{x: 0, y: 0, w: SIZE as u16, h: SIZE as u16},
            &gfx::Frame::new(SIZE as u16, SIZE as u16),
            gfx::Rect{x: 0, y: 0, w: SIZE as u16, h: SIZE as u16},
            gfx::MIRROR_Y,
            gfx::COLOR
        );

        graphics.end_frame();
        window.swap_buffers();
    }
}
