use core_graphics::color_space::CGColorSpace;
use core_graphics::sys::{CGContextRef, CGImageRef};
use core_graphics::{
    base::{
        kCGBitmapByteOrder32Little, kCGImageAlphaPremultipliedFirst, kCGImageAlphaPremultipliedLast,
    },
    context::CGContext,
    image::CGImageAlphaInfo,
};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::ffi::c_void;
use std::{
    arch::aarch64::int16x4x3_t,
    time::{Duration, Instant},
    vec,
};
use winit::raw_window_handle::{HasRawWindowHandle, RawWindowHandle, WindowHandle};
use winit::{
    self,
    application::ApplicationHandler,
    event::{self, Event, WindowEvent},
    event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder},
    window::{self, Window, WindowAttributes},
};

use crate::types::*;
use crate::triangle::*;
use crate::rasterizer::*;
mod rasterizer;
mod triangle;
mod types;

struct App {
    window: Option<Window>,
    last_frame: Instant,
    layer: Option<*mut objc::runtime::Object>,
    context: Option<CGContext>,
    buffer: Option<Vec<u8>>,
    width: usize,
    height: usize,
    t: f32,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let w_size: u32 = 1000;
        let h_size: u32 = 1000;
        let mut buffer = vec![0u8; self.width * self.height * 4];
        println!("{}", buffer.len());
        let color_space = CGColorSpace::create_device_rgb();
        let bitmap_info = kCGImageAlphaPremultipliedFirst | kCGBitmapByteOrder32Little;
        let ctx = CGContext::create_bitmap_context(
            Some(buffer.as_mut_ptr() as *mut c_void),
            self.width,
            self.height,
            8,
            self.width * 4,
            &color_space,
            bitmap_info,
        );
        let attributes = WindowAttributes::default();
        let window = event_loop.create_window(Default::default()).unwrap();
        window.set_max_inner_size(Some(winit::dpi::LogicalSize::new(w_size, h_size)));
        //gets window handle for blitting later
        let handle = window.raw_window_handle().unwrap();
        let ns_view = match handle {
            RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast(),
            _ => panic!("exit"),
        };
        let ns_view: *mut Object = ns_view as *mut Object;
        let layer: *mut Object = unsafe { msg_send![ns_view, layer] };
        let scale: f64 = unsafe { msg_send![ns_view, backingScaleFactor] };
        unsafe {
            let _: () = msg_send![ns_view, setWantsLayer: true];
            let _: () = msg_send![layer, setContentsScale: scale];
        };
        self.layer = Some(layer);
        self.buffer = Some(buffer);
        self.context = Some(ctx);
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta = now - self.last_frame;
                let dt = delta.as_secs_f32();
                let point = Point2 { x: -50, y: -50, color: Color { b: 255, g: 0, r: 0, a: 255 } };
                set_background(&mut self.buffer.as_mut().unwrap(), self.width, self.height, Color {b: 0, g: 0, r: 255, a: 255});
                if now - self.last_frame >= Duration::from_millis(16) {
                    self.last_frame = now;
                    //rendering performed inside here, limits framerate

                    /*
                    //buffer update each frame
                    for y in 0..self.height {
                        for x in 0..self.width {
                            let fx = x as f32 / self.width as f32;
                            let fy = y as f32 / self.width as f32;
                            let r = (((fx + self.t).sin() * 0.5 + 0.5) * 255.0) as u8;
                            let g = (((fy + self.t).cos() * 0.5 + 0.5) * 255.0) as u8;
                            let b = 255 as u8;
                            set_pixel(
                                &mut self.buffer.as_mut().unwrap(),
                                self.width,
                                x,
                                y,
                                b,
                                g,
                                r,
                                255,
                            );
                        }
                    }
                     */
                    /* 
                    match screen_point {
                        Some(value) => println!("point.x: {}, point.y: {}", value.0, value.1),
                        None => println!("no value found"),
                    }
                    */
                    set_pixel(
                        &mut self.buffer.as_mut().unwrap(),
                        self.width,
                        self.height,
                        &point,
                    );
                    let vec1 = Point2 {x: 100, y: 100, color: Color { b: 0, g: 0, r: 0, a: 255 }};
                    let vec2 = Point2 {x: -100, y: -131, color: Color { b: 0, g: 0, r: 0, a: 255 }};
                    let vec3 = Point2 {x: 100, y: -100, color: Color { b: 0, g: 0, r: 0, a: 255 }};
                    
                    let triangle = Triangle::new(vec1, vec2, vec3);
                    triangle.draw(self.buffer.as_mut().unwrap(), self.width, self.height, true);
                    //remaking CGImage each frame
                    let image = self.context.as_ref().unwrap().create_image().unwrap();
                    unsafe {
                        let _: () = msg_send![self.layer.unwrap(), setContents: image];
                    }

                    self.t += dt;
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            _ => (),
        }
    }
}
fn set_background(buffer: &mut Vec<u8>, width: usize, height: usize, color: Color) {
    for y in 0..height{
        for x in 0..width{
            let i = (y * width + x) * 4;
            buffer[i] = color.b;
            buffer[i + 1] = color.g;
            buffer[i + 2] = color.r;
            buffer[i + 3] = color.a;
        }
    }
}
fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        last_frame: Instant::now(),
        buffer: None,
        layer: None,
        context: None,
        width: 1000,
        height: 1000,
        t: 0.0,
    };
    let cam = Camera::new([12, 13, 10], [11, 10, 9]);
    event_loop.run_app(&mut app).expect("could not run app");
}
