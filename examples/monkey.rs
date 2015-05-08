#![feature(core)]


extern crate rusterize;
extern crate genmesh;
extern crate image;
extern crate obj;
extern crate cgmath;
extern crate time;
extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use piston::window::WindowSettings;
use piston::event::*;
use piston_window::*;
use graphics::{clear};
use sdl2_window::{ Sdl2Window };
use opengl_graphics::*;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use genmesh::{Triangulate, MapToVertices};
use rusterize::{Frame, Fragment, Raster};
use image::{ImageBuffer, Rgba};
use cgmath::*;
use time::precise_time_s;

const SIZE: u32 = 1024;

fn main() {
    let window = Rc::new(RefCell::new(
        Sdl2Window::new(
            OpenGL::_3_2,
            WindowSettings::new(
                "rusterize: image_test",
                [1024, 1024]
            )
            .exit_on_esc(true)
        )
    ));

    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let light_normal = Vector4::new(10., 10., 10., 0.).normalize();
    let kd = Vector4::new(64., 128., 64., 1.);
    let ka = Vector4::new(16., 16., 16., 1.);

    let proj = cgmath::perspective(cgmath::deg(60.0f32), 1.0, 0.01, 100.0);
    let mut frame = Frame::new(SIZE, SIZE, Rgba([0u8, 0, 0, 0]));

    let mut raster_order = false;
    let mut paused = false;

    let mut gl = GlGraphics::new(OpenGL::_3_2);
    let mut img = ImageBuffer::new(1024, 1024);
    let mut texture = Texture::from_image(&img);

    let mut last = precise_time_s();

    let mut time = precise_time_s();
    for e in  window.events() {
        //glfw.poll_events();
        /*for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    canvas.output.window.set_should_close(true),
                glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) =>
                    paused ^= true,
                glfw::WindowEvent::Key(glfw::Key::R, _, glfw::Action::Press, _) =>
                    raster_order ^= true,
                _ => {},
            }
        }*/

        if paused {
            continue;
        }


        if let Some(args) = e.render_args() {
            let next = precise_time_s();
            println!("{}", (next - time) * 1000.);
            time = next;
            let cam_pos = {
                // Slowly circle the center
                let x = (0.25*time as f32).sin();
                let y = (0.25*time as f32).cos();
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
            struct RO {
                v: Arc<AtomicUsize>
            }

            impl Fragment<[f32; 4]> for RO {
                type Color = Rgba<u8>;

                #[inline]
                fn fragment(&self, _ : [f32; 4]) -> Rgba<u8> {
                    let x = self.v.fetch_add(1, Ordering::SeqCst);
                    Rgba([0, (x >> 12) as u8, (x >> 18) as u8, 255])
                }
            }

            let start = precise_time_s();
            frame.clear(Rgba([0u8, 0, 0, 0]));
            if !raster_order {
                frame.raster(vertex, V{ka: ka, kd: kd, light_normal: light_normal});
            } else {
                frame.raster(vertex.vertex(|(p, _)| { p }), RO{v: Arc::new(AtomicUsize::new(0))});
            }
            let raster = precise_time_s();

            img = frame.into_image(img);
            texture.update(&img);

            let texture_done = precise_time_s();
            let transform = graphics::math::abs_transform(1024 as f64, 1024 as f64);

            gl.draw(args.viewport(), |c, g| {
                use graphics::*;
                clear([0.0; 4], g);
                image(&texture, transform, g);
            });

            let last = precise_time_s();

            println!("{} {} {}",
                (raster - start) * 1000.,
                (texture_done - raster) * 1000.,
                (last - texture_done) * 1000.
            );
        }
    }
}
