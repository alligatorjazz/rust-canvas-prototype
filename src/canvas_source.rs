extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::utils::set_panic_hook;
// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct CanvasSource {
    width: u32,
    height: u32,
    data: Vec<u8>,
    ctx: WebGl2RenderingContext,
}

fn compile_shader(
    source: &mut CanvasSource,
    shader_type: u32,
    shader_source: &str,
) -> Result<WebGlShader, String> {
    console_log!("{:?}", source.ctx);
    console_log!("rs (compile_shader): fetched context from source");

    let shader = source
        .ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    source.ctx.shader_source(&shader, shader_source);
    source.ctx.compile_shader(&shader);

    if source
        .ctx
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(source
            .ctx
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

#[derive(Debug)]
pub struct UniformLocations {
    projection_matrix: WebGlUniformLocation,
    model_view_matrix: WebGlUniformLocation,
}

#[derive(Debug)]
pub struct AttributeLocations {
    vertexPosition: i32,
}

#[derive(Debug)]
pub struct ProgramInfo {
    program: WebGlProgram,
    uniform_locations: UniformLocations,
    attrib_locations: AttributeLocations,
}

fn initialize_shader_program(source: &mut CanvasSource) -> ProgramInfo {
    console_log!("rs run(): running...");
    // loads shaders from local files
    let vertex_shader = compile_shader(
        source,
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

    let fragment_shader = compile_shader(
        source,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"
			void main() {
				gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
			}
		"##,
    )
    .expect("Failed to load fragment shader.");

    console_log!("rs run(): successfully loaded shaders");
    let shader_program = source
        .ctx
        .create_program()
        .expect("Failed to load shader program.");

    source.ctx.attach_shader(&shader_program, &vertex_shader);
    source.ctx.attach_shader(&shader_program, &fragment_shader);

    source.ctx.link_program(&shader_program);

    ProgramInfo {
		attrib_locations: AttributeLocations { 
			vertexPosition: source.ctx.get_attrib_location(&shader_program, "aVertexPosition") 
		},
		uniform_locations: UniformLocations {
			projection_matrix: 
				source.ctx.get_uniform_location(&shader_program, "uProjectionMatrix").expect("Could not get projection matrix."),
			model_view_matrix: source.ctx.get_uniform_location(&shader_program, "uModelViewMatrix").expect("Could not get model view matrix."),
		},
		program: shader_program		
    }
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

    pub fn init(&mut self) {
		console_log!("{:?}", initialize_shader_program(self))
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
