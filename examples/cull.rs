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
extern crate rand;

use gfx::traits::*;
use glfw::Context;
use genmesh::*;
use rusterize::{Frame, Fragment, Barycentric};
use image::Rgba;
use cgmath::*;
use time::precise_time_s;
use std::num::Float;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::distributions::{IndependentSample, Range};

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

    let mut frame = Frame::new(SIZE, SIZE);
    let texture = graphics.device.create_texture(texture_info).unwrap();

    let mut texture_frame = gfx::Frame::new(SIZE as u16, SIZE as u16);
    texture_frame.colors.push(gfx::Plane::Texture(texture.clone(), 0, None));

    let mut show_grid = 0;
    let mut raster_order = false;
    let mut paused = false;

    let mut tri = Triangle::new(
        Vector2::new(-0.5, -0.5),
        Vector2::new( 0.5, -0.5),
        Vector2::new( 0.0,  0.5),
    ).map_vertex(|v| {
        Vector2::new(v.x * 512. + 512., v.y * 512. + 512.)
    });
    let mut bary = Barycentric::new(tri);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    window.set_should_close(true),
                glfw::WindowEvent::Key(glfw::Key::Num1, _, glfw::Action::Press, _) =>
                    show_grid = if show_grid == 8 { 0 } else { 8 },
                glfw::WindowEvent::Key(glfw::Key::Num2, _, glfw::Action::Press, _) =>
                    show_grid = if show_grid == 16 { 0 } else { 16 },
                glfw::WindowEvent::Key(glfw::Key::Num3, _, glfw::Action::Press, _) =>
                    show_grid = if show_grid == 32 { 0 } else { 32 },
                glfw::WindowEvent::Key(glfw::Key::Num4, _, glfw::Action::Press, _) =>
                    show_grid = if show_grid == 64 { 0 } else { 64 },
                glfw::WindowEvent::Key(glfw::Key::Num5, _, glfw::Action::Press, _) =>
                    show_grid = if show_grid == 128 { 0 } else { 128 },
                glfw::WindowEvent::Key(glfw::Key::Num6, _, glfw::Action::Press, _) =>
                    show_grid = if show_grid == 256 { 0 } else { 256 },
                glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
                    let between = Range::new(-1f32, 1.);
                    let mut rng = rand::thread_rng();
                    tri = Triangle::new(
                        Vector2::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng)),
                        Vector2::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng)),
                        Vector2::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng)),
                    ).map_vertex(|v| {
                        Vector2::new(v.x * 512. + 512., v.y * 512. + 512.)
                    });
                    bary = Barycentric::new(tri);
                }
                glfw::WindowEvent::Key(glfw::Key::R, _, glfw::Action::Press, _) =>
                    raster_order ^= true,
                _ => {},
            }
        }

        let proj = ortho(-1., 1., -1., 1., -2., 2.);

        let plane = generators::Plane::new()
            .triangulate()
            .vertex(|v| proj.mul_v(&Vector4::new(v.0, v.1, 1., 1.)).into_fixed())
            .vertex(|v| {
                (v, [512. * v[0] / v[3] + 512., 512. * v[1] / v[3] + 512.])
            });

        #[derive(Clone)]
        struct V {
            bary: Barycentric
        }

        impl Fragment<([f32; 4], [f32; 2])> for V {
            type Color = Rgba<u8>;

            fn fragment(&self, (pos, screen) : ([f32; 4], [f32; 2])) -> Rgba<u8> {
                let coord = self.bary.coordinate(Vector2::new(screen[0], screen[1]));

                let x0 = screen[0] as u32 & !0x7;
                let y0 = screen[1] as u32 & !0x7;
                let x1 = screen[0] as u32 & !0xF;
                let y1 = screen[1] as u32 & !0xF;
                let x2 = screen[0] as u32 & !0x1F;
                let y2 = screen[1] as u32 & !0x1F;
                let x3 = screen[0] as u32 & !0x3F;
                let y3 = screen[1] as u32 & !0x3F;

                let b = if coord.inside() { 255 } else { 0 };

                if !self.bary.tile_covered(Vector2::new(x3 as f32, y3 as f32), Vector2::new(63., 63.)) {
                    Rgba([0, 196, b, 255])
                } else if !self.bary.tile_covered(Vector2::new(x2 as f32, y2 as f32), Vector2::new(31., 31.)) {
                    Rgba([0, 128, b, 255])
                } else if !self.bary.tile_covered(Vector2::new(x1 as f32, y1 as f32), Vector2::new(15., 15.)) {
                    Rgba([0, 64, b, 255])
                } else if !self.bary.tile_covered(Vector2::new(x0 as f32, y0 as f32), Vector2::new(7., 7.)) {
                    Rgba([0, 32, b, 255])
                } else if !self.bary.tile_fast_check(Vector2::new(x0 as f32, y0 as f32), Vector2::new(7., 7.)) {
                    Rgba([196, 0, b, 255])
                } else if !self.bary.tile_fast_check(Vector2::new(x1 as f32, y1 as f32), Vector2::new(15., 15.)) {
                    Rgba([128, 0, b, 255])
                } else if !self.bary.tile_fast_check(Vector2::new(x2 as f32, y2 as f32), Vector2::new(31., 31.)) {
                    Rgba([64, 0, b, 255])
                } else if !self.bary.tile_fast_check(Vector2::new(x3 as f32, y3 as f32), Vector2::new(63., 63.)) {
                    Rgba([32, 0, b, 255])
                } else {
                    Rgba([0, 0, 0, 255])
                }
            }
        }

        let mut frame = Frame::new(SIZE, SIZE);
        frame.raster(plane, V{bary: bary});

        if show_grid != 0 {
            frame.draw_grid(show_grid, Rgba([128, 128, 128, 255]));
        }
        graphics.device.update_texture(&texture, &image_info, frame.to_image().as_slice()).unwrap();

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
