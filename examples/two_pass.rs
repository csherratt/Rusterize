#![feature(core, path)]

extern crate gfx;
extern crate gfx_device_gl;
extern crate glfw;
extern crate rusterize;
extern crate genmesh;
extern crate image;
extern crate obj;
extern crate cgmath;
extern crate time;
extern crate timeout_iter;

use std::num::Float;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::Path;

use gfx::traits::*;
use glfw::Context;
use genmesh::*;
use rusterize::{Frame, Fragment, Flat, Mapping, Interpolate};
use image::Rgba;
use cgmath::*;
use time::precise_time_s;
use timeout_iter::TimeoutIter;


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

    let device = gfx_device_gl::GlDevice::new(|s| window.get_proc_address(s));
    let mut graphics = device.into_graphics();

    let texture_info = gfx::tex::TextureInfo {
        width: SIZE as u16, height: SIZE as u16, depth: 1, levels: 1,
        kind: gfx::tex::TextureKind::Texture2D,
        format: gfx::tex::Format::Unsigned(gfx::tex::Components::RGBA, 8, gfx::attrib::IntSubType::Normalized)
    };
    let image_info = texture_info.to_image_info();

    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let light_normal = Vector4::new(10., 10., 10., 0.).normalize();
    let kd = Vector4::new(64., 128., 64., 1.);
    let ka = Vector4::new(16., 16., 16., 1.);

    let proj = cgmath::perspective(cgmath::deg(60.0f32), 1.0, 0.01, 100.0);
    let mut frame = Frame::new(SIZE, SIZE, Rgba([0u8, 0, 0, 0]));

    //let proj = cgmath::ortho(-1., 1., -1., 1., -0.8, 0.8);
    let mut frame = Frame::new(SIZE, SIZE, (None, 0., 0.));
    let mut frame_dst = Frame::new(SIZE, SIZE, Rgba([0u8, 0, 0, 0]));

    let texture = graphics.device.create_texture(texture_info).unwrap();

    let mut texture_frame = gfx::Frame::new(SIZE as u16, SIZE as u16);
    texture_frame.colors.push(gfx::Plane::Texture(texture.clone(), 0, None));

    let mut show_grid = 0;
    let mut raster_order = false;
    let mut paused = false;


    let vertex: Vec<Triangle<([f32; 3], [f32; 3])>> = monkey.indices().iter().map(|x| *x)
                           .vertex(|(p, _, n)| { (obj.position()[p], obj.normal()[n.unwrap()]) })
                           .triangulate()
                           .collect();
    let vertex = Arc::new(vertex);

    let mut i = 0;

    drop(monkey);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    window.set_should_close(true),
                glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) =>
                    paused ^= true,
                glfw::WindowEvent::Key(glfw::Key::R, _, glfw::Action::Press, _) =>
                    raster_order ^= true,
                _ => {},
            }
        }

        if paused {
            continue;
        }


        let time = precise_time_s() as f32;
        let cam_pos = {
            // Slowly circle the center
            let x = (0.25*time).sin();
            let y = (0.25*time).cos();
            Point3::new(x * 1.0, y * 1.0, 1.0)
        };
        let view: AffineMatrix3<f32> = Transform::look_at(
            &cam_pos,
            &Point3::new(0.0, 0.0, 0.0),
            &Vector3::unit_y(),
        );
        let mat = proj.mul_m(&view.mat);

        let start = precise_time_s();
        let vd: Vec<Triangle<([f32; 4], [f32; 3])>> = vertex.iter().map(|x| *x)
                           .vertex(|(p, n)| (mat.mul_v(&Vector4::new(p[0], p[1], p[2], 1.)).into_fixed(), n))
                           .triangulate().collect();
        let end = precise_time_s();

        println!("vs {}ms", (end - start) * 1000.);

        #[derive(Clone)]
        struct Index;

        impl Fragment<([f32; 4], [f32; 3], u32)> for Index {
            type Color = (Option<u32>, f32, f32);

            #[inline]
            fn fragment(&self, (_, uv, i) : ([f32; 4], [f32; 3], u32)) -> (Option<u32>, f32, f32) {
                (Some(i), uv[1], uv[2])
            }
        }

        #[derive(Clone)]
        struct V {
            ka: Vector4<f32>,
            kd: Vector4<f32>,
            light_normal: Vector4<f32>
        }

        impl Fragment<([f32; 4], [f32; 3])> for V {
            type Color = Rgba<u8>;

            #[inline]
            fn fragment(&self, (_, n) : ([f32; 4], [f32; 3])) -> Rgba<u8> {
                let normal = Vector4::new(n[0], n[1], n[2], 0.);
                let v = self.kd.mul_s(self.light_normal.dot(&normal).partial_max(0.)) + self.ka;
                Rgba([v.x as u8, v.y as u8, v.z as u8, 255])
            }
        }

        #[derive(Clone)]
        struct Shader {
            ka: Vector4<f32>,
            kd: Vector4<f32>,
            light_normal: Vector4<f32>,
            vertex: Arc<Vec<Triangle<([f32; 3], [f32; 3])>>>
        }

        impl Mapping<(Option<u32>, f32, f32)> for Shader {
            type Out = Rgba<u8>;

            #[inline]
            fn mapping(&self, (p, a, b): (Option<u32>, f32, f32)) -> Rgba<u8> {
                match p {
                    Some(v) => {
                        let n = self.vertex[v as usize].map_vertex(|(_, n)| n);
                        let n = Interpolate::interpolate(&n, [1. - (a + b), a, b]);
                        let normal = Vector4::new(n[0], n[1], n[2], 0.);
                        let v = self.kd.mul_s(self.light_normal.dot(&normal).partial_max(0.)) + self.ka;
                        Rgba([v.x as u8, v.y as u8, v.z as u8, 255])
                    }
                    None => Rgba([0, 0, 0, 0])
                }
            }
        }


        let start = precise_time_s();
        frame.clear((None, 0., 0.));

        let mut index: u32 = 0;
        frame.raster(vd.iter().map(|t| {
                                        let i = index;
                                        index += 1;
                                        Triangle::new((t.x.0, [0., 0., 0.], Flat(i)),
                                                      (t.y.0, [0., 1., 0.], Flat(i)),
                                                      (t.z.0, [0., 0., 1.], Flat(i)))
                                    }), Index);
        frame_dst.map(&mut frame, Shader{
            vertex: vertex.clone(),
            ka: ka,
            kd: kd,
            light_normal: light_normal
        });
        graphics.device.update_texture(&texture, &image_info, frame_dst.to_image().as_slice()).unwrap();
        let end = precise_time_s();
        println!("{}ms", (end - start) * 1000.);

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
