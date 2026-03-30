
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::ffi::c_void;
use std::time::{Duration, Instant};
use winit::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::{
    self,
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{self, Window, WindowAttributes},
};
use std::ptr;
unsafe extern "C" {
    fn IOSurfaceCreate(properties: *const c_void) -> *mut c_void;
    fn IOSurfaceLock(surface: *mut c_void, options: u32, seed: *mut u32) -> i32;
    fn IOSurfaceUnlock(surface: *mut c_void, options: u32, seed: *mut u32) -> i32;
    fn IOSurfaceGetBaseAddress(surface: *mut c_void) -> *mut c_void;
}

use crate::rectangle::Rectangle;
use crate::types::*;
use crate::triangle::*;
mod rasterizer;
mod triangle;
mod types;
mod rectangle;

struct App {
    window: Option<Window>,
    last_frame: Instant,
    layer: Option<*mut objc::runtime::Object>,
    surfaces: Option<[*mut c_void; 2]>,
    current: usize,
    width: usize,
    height: usize,
    t: f32,
    triangles: Vec<Triangle>
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {


        let attributes = WindowAttributes::default();
        let window = event_loop.create_window(WindowAttributes::default().with_inner_size(
            winit::dpi::LogicalSize::new(self.width as u32, self.height as u32)
        )).unwrap();
        window.set_max_inner_size(Some(winit::dpi::LogicalSize::new(self.width as u32, self.height as u32)));
        //gets window handle for blitting later
        let handle = window.raw_window_handle().unwrap();

        let ns_view = match handle {
            RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast(),
            _ => panic!("exit"),
        };
        let ns_view: *mut Object = ns_view as *mut Object;

        let layer: *mut Object = unsafe { msg_send![ns_view, layer] };

        let scale: f64 = unsafe { msg_send![ns_view, backingScaleFactor] };

        let surface_a = unsafe{create_surface(self.width, self.height)};
        let surface_b = unsafe{create_surface(self.width, self.height)};

        unsafe {
            //display a
            let _:() = msg_send![layer, setContents: surface_a as *mut Object ];

            let _: () = msg_send![ns_view, setWantsLayer: true];
            let _: () = msg_send![layer, setContentsScale: 1.0f64];
            let _: () = msg_send![layer, setContents: surface_a];
        }
        
        self.layer = Some(layer);
        self.surfaces = Some([surface_a, surface_b]);
        //write to b (a is being displayed)
        self.current = 1;
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
                    if let Some(surfaces) = self.surfaces{
                        
                        let back = surfaces[self.current];

                        unsafe{ IOSurfaceLock(back, 0, ptr::null_mut());}
                        
                        let base = unsafe{ IOSurfaceGetBaseAddress(back)};
                        let buffer = unsafe{
                            std::slice::from_raw_parts_mut(base as *mut u8, self.width * self.height * 4)
                        };


                        
                        
                        set_background(buffer, self.width, self.height, Color {b:0, g:0, r:255, a:255});
                        
                        let offset = (self.t * 2.0).sin();
                        let d_x = (100.0 * offset) as i32;
                        let d_y = (100.0) as i32;
                        let vec1 = Point2 {x: d_x, y: d_y, color: Color { b: 0, g: 0, r: 0, a: 255 }};
                        let vec2 = Point2 {x: d_x, y: -d_y, color: Color { b: 0, g: 0, r: 0, a: 255 }};
                        let vec3 = Point2 {x: -d_x, y: -d_y, color: Color { b: 0, g: 0, r: 0, a: 255 }};
                        let vec4 = Point2 {x: -d_x, y: d_y, color: Color {b: 0, g: 0, r: 0, a: 255}};
                    
                        let rectangle = Rectangle::new(vec1, vec2, vec3, vec4,);
                        rectangle.draw(buffer, self.width, self.height, true);
                        

                        unsafe { IOSurfaceUnlock(back, 0, ptr::null_mut()); }

                        unsafe {    
                            let _:() = msg_send![self.layer.unwrap(), setContents: back as *mut Object];
                        }

                        //next frame we write to other surface
                        self.current = 1 - self.current
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
unsafe fn create_surface(width: usize, height: usize) -> *mut c_void{

    let properties: *mut Object = unsafe { msg_send![objc::class!(NSMutableDictionary), new] };
    //we are doing magic here
    //raw objc pointers
    let w: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: width as i32];
    let h: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: height as i32];
    let bpe: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: 4i32];
    let bpr: *mut Object = msg_send![objc::class!(NSNumber), numberWithInt: (width * 4) as i32];
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

    let surface =  IOSurfaceCreate(properties as *const c_void);
    assert!(!surface.is_null(), "IOSurface create failed");
    surface

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
        surfaces: None,
        current: 0,
        width: 1000,
        height: 1000,
        t: 0.0,
        triangles: Vec::new()
    };
    let cam = Camera::new([12, 13, 10], [11, 10, 9]);
    event_loop.run_app(&mut app).expect("could not run app");
}
