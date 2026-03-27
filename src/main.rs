
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
use std::ptr;
unsafe extern "C" {
    fn IOSurfaceCreate(properties: *const c_void) -> *mut c_void;
    fn IOSurfaceLock(surface: *mut c_void, options: u32, seed: *mut u32) -> i32;
    fn IOSurfaceUnlock(surface: *mut c_void, options: u32, seed: *mut u32) -> i32;
    fn IOSurfaceGetBaseAddress(surface: *mut c_void) -> *mut c_void;
}

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
    surface: Option<*mut c_void>,
    width: usize,
    height: usize,
    t: f32,
    triangles: Vec<Triangle>
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let vec1 = Point2 {x: 100, y: 100, color: Color { b: 0, g: 0, r: 0, a: 255 }};
        let vec2 = Point2 {x: -100, y: -131, color: Color { b: 0, g: 0, r: 0, a: 255 }};
        let vec3 = Point2 {x: 100, y: -100, color: Color { b: 0, g: 0, r: 0, a: 255 }};
                    
        let triangle = Triangle::new(vec1, vec2, vec3);

        let mut triangles: Vec<Triangle> = Vec::new();
        triangles.push(triangle);

        self.triangles = triangles;

        let w_size: u32 = 1000;
        let h_size: u32 = 1000;
        
        let properties: *mut Object = unsafe { msg_send![objc::class!(NSMutableDictionary), new] };
        unsafe {
            //we are doing magic here
            //raw objc pointers
            let w: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: self.width as i32];
            let h: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: self.height as i32];
            let bpe: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: 4i32];
            let bpr: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: (self.width * 4) as i32];
            let pf: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: 0x42475241i32]; //magic number, pixel format, ARGB little endian

            let k_width: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String: b"IOSurfaceWidth\0".as_ptr()];
            let k_height: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String: b"IOSurfaceHeight\0".as_ptr()];
            let k_bpe: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String: b"IOSurfaceBytesPerElement\0".as_ptr()];
            let k_bpr: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String: b"IOSurfaceBytesPerRow\0".as_ptr()];
            let k_pf: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String: b"IOSurfacePixelFormat\0".as_ptr()];

            let _: () = msg_send![properties, setObject: w forKey: k_width];
            let _: () = msg_send![properties, setObject: h forKey: k_height];
            let _: () = msg_send![properties, setObject: bpe forKey: k_bpe];
            let _: () = msg_send![properties, setObject: bpr forKey: k_bpr];
            let _: () = msg_send![properties, setObject: pf forKey: k_pf];
        };
        let surface = unsafe { IOSurfaceCreate(properties as *const c_void)};

        if surface.is_null() {
            panic!("IOSurfaceCreate returned null");
        }

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
            let _: () = msg_send![layer, setContentsScale: 1.0f64];
            let _: () = msg_send![layer, setContents: surface];
        };

        self.layer = Some(layer);
        self.surface = Some(surface);
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
                
                if now - self.last_frame >= Duration::from_millis(16) {
                    self.last_frame = now;
                    //rendering performed inside here, limits framerate

                    if let Some(surface) = self.surface{
                        
                        unsafe{ IOSurfaceLock(surface, 0, ptr::null_mut());}
                        
                        let base = unsafe{ IOSurfaceGetBaseAddress(surface)};
                        let buffer = unsafe{
                            std::slice::from_raw_parts_mut(base as *mut u8, self.width * self.height * 4)
                        };
                        

                        set_background(buffer, self.width, self.height, Color {b: 0, g: 0, r: 255, a: 255});

                        for triangle in &self.triangles{
                        triangle.draw(buffer, self.width, self.height, true);
                        }

                        unsafe { IOSurfaceUnlock(surface, 0, ptr::null_mut()); }

                        unsafe {    
                            let _:() = msg_send![self.layer.unwrap(), setContents: surface];
                        }
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
fn set_background(buffer: &mut [u8], width: usize, height: usize, color: Color) {
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
        layer: None,
        surface: None,
        width: 1000,
        height: 1000,
        t: 0.0,
        triangles: Vec::new()
    };
    let cam = Camera::new([12, 13, 10], [11, 10, 9]);
    event_loop.run_app(&mut app).expect("could not run app");
}
