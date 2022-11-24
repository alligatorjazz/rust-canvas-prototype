#[cfg(web_sys_unstable_apis)]
use wasm_bindgen::prelude::*;
extern crate fixedbitset;
extern crate web_sys;

// TODO: implement wgpu here
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winit::window::Window;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

// impl State {
//     // Creating some of the wgpu types requires async code
//     async fn new(window: &Window) -> Self {
//         let size = window.inner_size();

//         // INFO: may not work on all machines: see learn wgpu "Instance and Adapter"
//         // TODO: automatically find the best adapter for a given machine
//         // The instance is a handle to our GPU
//         // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
//         let instance = wgpu::Instance::new(wgpu::Backends::all());
//         let surface = unsafe { instance.create_surface(window) };
//         let adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::default(),
//                 compatible_surface: Some(&surface),
//                 force_fallback_adapter: false,
//             })
//             .await
//             .unwrap();

//         let (device, queue) = adapter
//             .request_device(
//                 &wgpu::DeviceDescriptor {
//                     features: wgpu::Features::empty(),
//                     // WebGL doesn't support all of wgpu's features, so if
//                     // we're building for the web we'll have to disable some.
//                     limits: if cfg!(target_arch = "wasm32") {
//                         wgpu::Limits::downlevel_webgl2_defaults()
//                     } else {
//                         wgpu::Limits::default()
//                     },
//                     label: None,
//                 },
//                 None, // Trace path
//             )
//             .await
//             .unwrap();

// 			let config = wgpu::SurfaceConfiguration {
// 				usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
// 				format: surface.get_supported_formats(&adapter)[0],
// 				width: size.width,
// 				height: size.height,
// 				present_mode: wgpu::PresentMode::Fifo,
// 				alpha_mode: wgpu::CompositeAlphaMode::Auto,
// 			};
// 			surface.configure(&device, &config);
//     }

//     fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         todo!()
//     }

//     fn input(&mut self, event: &WindowEvent) -> bool {
//         todo!()
//     }

//     fn update(&mut self) {
//         todo!()
//     }

//     fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//         todo!()
//     }
// }

use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub fn run() {
    set_panic_hook();
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}

#[wasm_bindgen]
pub struct CanvasSource {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl CanvasSource {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // returns pointer to canvas image data
    pub fn data(&self) -> *const u8 {
        self.data.as_ptr()
    }

    // take in data and start placing pixels from the top right
    // pub fn scale_to_source(data: Vec<u8>) {}

    // take in data and start placing pixels from a given location
    // pub fn scale_to_source_from_offset(data: Vec<u8>, h_offset: u32, y_offset: u32) {}

    pub fn cover_in_blood(&mut self) {
        let blood: Vec<u8> = vec![252, 3, 27, 255];
        let mut new_data = blood.clone();
        for _ in 0..self.width {
            for _ in 0..self.height {
                let pixel = blood.clone();
                new_data = [new_data, pixel].concat()
            }
        }

        self.data = new_data;
    }

    pub fn new(width: u32, height: u32, initial_data: Vec<u8>) -> CanvasSource {
        let data_size = (width * height) as usize;
        let mut data = initial_data;
        data.resize(data_size, 0);

        CanvasSource {
            width,
            height,
            data,
        }
    }
}
