extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::utils::set_panic_hook;
// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct CanvasSource {
    width: u32,
    height: u32,
    data: Vec<u8>,
    ctx: WebGl2RenderingContext,
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

    pub fn cover_in_blood(&mut self) {
        let mut blood: Vec<u8> = vec![252, 3, 27, 255];
        let pixel = blood.clone();

        for _ in 0..self.width {
            for _ in 0..self.height {
                blood.extend(&pixel)
            }
        }

        self.data = blood;
    }

    fn compile_shader(&mut self, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        console_log!("{:?}", self.ctx);
        console_log!("rs (compile_shader): fetched context from source");

        let shader = self
            .ctx
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        self.ctx.shader_source(&shader, source);
        self.ctx.compile_shader(&shader);

        if self
            .ctx
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .ctx
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn run(&mut self) -> WebGlProgram {
        console_log!("rs run(): running...");
        // loads shaders from local files

        let vertex_shader = self
            .compile_shader(
                WebGl2RenderingContext::VERTEX_SHADER,
                r##"
					attribute vec4 aVertexPosition;

					uniform mat4 uModelViewMatrix;
					uniform mat4 uProjectionMatrix;
					
					void main() {
						gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
					}				
				"##,
            )
            .expect("Failed to load vertex shader.");

        let fragment_shader = self
            .compile_shader(
                WebGl2RenderingContext::FRAGMENT_SHADER,
                r##"
					void main() {
						gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
					}
				"##,
            )
            .expect("Failed to load fragment shader.");

        console_log!("rs run(): successfully loaded shaders");
        let shader_program = self
            .ctx
            .create_program()
            .expect("Failed to load shader program.");

        self.ctx.attach_shader(&shader_program, &vertex_shader);
        self.ctx.attach_shader(&shader_program, &fragment_shader);

        self.ctx.link_program(&shader_program);

        shader_program
        // TODO: implement rest of MDN shader example
    }

    fn render_pass(&mut self) {
        todo!()
    }

    pub fn new(
        width: u32,
        height: u32,
        initial_data: Vec<u8>,
        ctx: WebGl2RenderingContext,
    ) -> CanvasSource {
        set_panic_hook();

        let data_size = (width * height) as usize;
        let mut data = initial_data;
        data.resize(data_size, 0);

        console_log!("testing to see if context is loaded w/ new: {:?}", ctx);
        console_log!("rs new(): successfully initialized source");
        CanvasSource {
            width,
            height,
            data,
            ctx,
        }
    }
}
