mod canvas_source;
mod universe;
mod utils;
// use wasm_bindgen::prelude::*;
extern crate fixedbitset;
extern crate web_sys;

// wgpu
use wasm_bindgen::prelude::*;
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use utils::set_panic_hook;
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

// lib.rs
impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// lib.rs
const PENTA_VERTEXES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];
const PENTA_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

const ROLY_VERTEXES: &[Vertex] = &[
    Vertex {
        position: [0.15, 0.2, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // A
    Vertex {
        position: [0.25, 0.1, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // B
    Vertex {
        position: [0.3, 0.0, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // C
    Vertex {
        position: [0.2, 0.0, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // D
    Vertex {
        position: [0.15, 0.1, 0.0],
        color: [1.0, 1.0, 1.0],
    }, // E
    Vertex {
        position: [0.1, 0.0, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // F
    Vertex {
        position: [0.0, 0.1, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // G
];

const ROLY_INDICES: &[u16] = &[
	0, 6, 1, 
	6, 5, 4, 
	4, 5, 3,
	4, 3, 1, 
	1, 3, 2
];

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    pentagon: bool,
    num_indices: u32,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // opts: LowPower, HighPerformance, default
                // (tries for LowPower if there is no HighPerformance)
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // The settings above work for most machines, but if there are
        // errors enumerate_adapters can loop through all available
        // adapters to find a suitable one.
        // let adapter = instance
        // 	.enumerate_adapters(wgpu::Backends::all())
        // 	.filter(|adapter| {
        // 		// Check if this adapter supports our surface
        // 		!surface.get_supported_formats(&adapter).is_empty()
        // 	})
        // 	.next()
        // 	.unwrap()

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        // similarly to above, adapter.features() / device.features() will
        // show the list of features available w/ a given graphics card.

        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let config = wgpu::SurfaceConfiguration {
            // Allows a texture to be an output attachment of a renderpass.
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // Gets supported formats straight from the adapter.
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            // Fifo = first in first out / Vsync on
            // To get a list of present modes, use surface.get_supported_modes(&adapter).
            present_mode: wgpu::PresentMode::Fifo,
            // opts: Opaque, Inherit, Auto
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            // continued ...
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // continued ...
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(PENTA_VERTEXES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        surface.configure(&device, &config);

        // NEW!
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(PENTA_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = PENTA_INDICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            pentagon: true,
            num_indices,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // input() returns a bool to indicate whether an event has been fully processed.
    // If the method returns true, the main loop won't process the event any further.
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    // TODO: We'll add some code here later on to move around objects.
    fn update(&mut self) {}

    fn change_shape(&mut self) {
        self.pentagon = !self.pentagon;
        let device = &self.device;
        // let surface = &self.surface;
        // let config = &self.config;

        if self.pentagon {
            self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(PENTA_VERTEXES),
                usage: wgpu::BufferUsages::VERTEX,
            });

            // surface.configure(device, config);

            // NEW!
            self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(PENTA_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
            self.num_indices = PENTA_INDICES.len() as u32;
        }
        if !self.pentagon {
            self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(ROLY_VERTEXES),
                usage: wgpu::BufferUsages::VERTEX,
            });

            // self.surface.configure(&device, &config);

            // NEW!
            self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(ROLY_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

            self.num_indices = ROLY_INDICES.len() as u32;
        }
    }
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // waits for the surface to provide a texture to render to
        let output = self.surface.get_current_texture()?;
        // creates a TextureView with default settings
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // encoder builds a buffer to store commands before sending them off the the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // 1.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }),
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.

            // When using an index buffer, you need to use draw_indexed. The draw method ignores
            // the index buffer. Also make sure you use the number of indices (num_indices), not
            // vertices as your model will either draw wrong, or the method will
            // panic because there are not enough indices.
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 2.
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[wasm_bindgen]
pub async fn run() {
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

    let mut state = State::new(&window).await;

    // run()
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
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
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        WindowEvent::MouseInput { button, .. } => {
                            if *button == MouseButton::Left {
                                state.change_shape()
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
