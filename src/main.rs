use std::{time::{Duration, Instant}, vec};
use winit::{self, application::ApplicationHandler, event::{self, Event, WindowEvent}, event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder}, window::{self, Window, WindowAttributes}};
use winit::raw_window_handle::{HasRawWindowHandle, WindowHandle, RawWindowHandle};
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use std::ffi::c_void;
use core_graphics::{context::CGContext,image::CGImageAlphaInfo, base::kCGImageAlphaPremultipliedFirst, base::kCGBitmapByteOrder32Little};
use core_graphics::color_space::CGColorSpace;
use core_graphics::sys::{CGContextRef, CGImageRef};

struct TPosition {
    x: i32,
    y: i32,
    z: i32, 
}
struct Pixel {
    position: TPosition,
    b: u8,
    r: u8,
    g: u8,
    a: u8,
}
struct Camera{
    position: TPosition,
    perspective: TPosition,
}
impl Camera{
    fn new(position: [i32; 3], perspective: [i32; 3]) -> Camera{
        Camera { position: TPosition { x: position[0] , y: position[1], z: position[2] }, perspective: TPosition { x: position[0] , y: position[1], z: position[2] }}
    }
}
struct App{
    window: Option<Window>,
    last_frame: Instant,
    layer: Option<*mut objc::runtime::Object>,
    context: Option<CGContext>,
    buffer: Option<Vec<u8>>,
    width: usize,
    height: usize,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop){
        let w_size:u32 = 800;
        let h_size:u32 = 600;
        let mut buffer = vec![0u8; self.width * self.height * 4];
        let color_space = CGColorSpace::create_device_rgb();
        let bitmap_info = kCGImageAlphaPremultipliedFirst
            | kCGBitmapByteOrder32Little;
        let ctx = CGContext::create_bitmap_context(
            Some(buffer.as_mut_ptr() as *mut c_void), 
            self.width, 
            self.height, 
            8, 
            self.width * 4, 
            &color_space, 
            bitmap_info
        ); 
        let attributes = WindowAttributes::default();
        let window = event_loop.create_window(Default::default()).unwrap();
        window.set_max_inner_size(Some(winit::dpi::LogicalSize::new(w_size, h_size)));
        //gets window handle for blitting later
        let handle = window.raw_window_handle().unwrap();
        let ns_view = match handle{
            RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast(),
            _ => panic!("exit"),
        };
        let ns_view: *mut Object = ns_view as *mut Object;
        let layer: *mut Object = unsafe {
            msg_send![ns_view, layer]
        };
        unsafe {
            let _: () = msg_send![ns_view, setWantsLayer: true];
        }

        for y in 0..self.height{
            for x in 0..self.width{
                let i = (y * self.width + x) * 4;
                buffer[i] = 0;//b
                buffer[i + 1] = 0;//g
                buffer[i + 2] = 255;//r
                buffer[i + 3] = 255; //a

            }
        }
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

                if now - self.last_frame >= Duration::from_millis(16){
                    self.last_frame = now;
                    //rendering performed inside here, limits framerate
                    let image = self.context.as_ref().unwrap().create_image().unwrap();
                    unsafe {
                        let _: () = msg_send![self.layer.unwrap(), setContents: image];
                    }

                }



                if let Some(window) = &self.window{
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

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App { window: None , last_frame: Instant::now() , buffer: None, layer: None, context: None, width: 600, height: 600};
    let cam = Camera::new([12, 13, 10], [11, 10, 9]);
    event_loop.run_app(&mut app).expect("could not run app");
}