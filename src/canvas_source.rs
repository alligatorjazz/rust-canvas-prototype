extern crate web_sys;

use std::{convert::TryInto, f32::consts::PI};

use gl_matrix::{common::Mat4, mat4};
use js_sys::Float64Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation,
};

use crate::utils::set_panic_hook;
// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug)]
pub struct UniformLocations {
    projection_matrix: WebGlUniformLocation,
    model_view_matrix: WebGlUniformLocation,
}

#[derive(Debug)]
pub struct AttributeLocations {
    vertex_position: i32,
}

#[derive(Debug)]
pub struct ProgramInfo {
    program: WebGlProgram,
    uniform_locations: UniformLocations,
    attrib_locations: AttributeLocations,
}

#[derive(Debug)]
pub struct NamedBuffer<'a> {
    name: &'a str,
    buffer: WebGlBuffer,
}

pub type NamedBufferList<'a> = Vec<NamedBuffer<'a>>;

trait SearchableBufferList {
    fn find(&self, name: &str) -> Option<&WebGlBuffer>;
}

impl SearchableBufferList for NamedBufferList<'_> {
    fn find(&self, name: &str) -> Option<&WebGlBuffer> {
        let result = self
            .iter()
            .find(|&data| data.name == name)
            .expect(format!("Buffer with name {} not found.", name).as_str());
        Some(&result.buffer)
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct CanvasSource {
    width: u32,
    height: u32,
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

    fn compile_shader(
        ctx: &WebGl2RenderingContext,
        shader_type: u32,
        shader_source: &str,
    ) -> Result<WebGlShader, String> {
        console_log!("rs (compile_shader): fetched context from source");

        let shader = ctx
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        ctx.shader_source(&shader, shader_source);
        ctx.compile_shader(&shader);

        if ctx
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(ctx
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn init_shader_program(ctx: &WebGl2RenderingContext) -> ProgramInfo {
        console_log!("rs init_shader_program(): running...");
        // loads shaders from local files
        let vertex_shader = CanvasSource::compile_shader(
            ctx,
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

        let fragment_shader = CanvasSource::compile_shader(
            ctx,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"
				void main() {
					gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
				}
			"##,
        )
        .expect("Failed to load fragment shader.");

        console_log!("rs run(): successfully loaded shaders");
        let shader_program = ctx
            .create_program()
            .expect("Failed to load shader program.");

        ctx.attach_shader(&shader_program, &vertex_shader);
        ctx.attach_shader(&shader_program, &fragment_shader);

        ctx.link_program(&shader_program);

        ProgramInfo {
            attrib_locations: AttributeLocations {
                vertex_position: ctx.get_attrib_location(&shader_program, "aVertexPosition"),
            },
            uniform_locations: UniformLocations {
                projection_matrix: ctx
                    .get_uniform_location(&shader_program, "uProjectionMatrix")
                    .expect("Could not get projection matrix."),
                model_view_matrix: ctx
                    .get_uniform_location(&shader_program, "uModelViewMatrix")
                    .expect("Could not get model view matrix."),
            },
            program: shader_program,
        }
    }

    fn init_buffers(ctx: &WebGl2RenderingContext) -> Vec<NamedBuffer> {
        // Create a buffer for the square's positions.
        let position_buffer = ctx.create_buffer().expect("Could not load position_buffer.");

        // Select the position_buffer as the one to apply buffer
        // operations to from here out.
        ctx.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            // as_ref() allows a borrow for a nested value
            Some(&position_buffer)
        );

        // Now pass the list of positions into WebGL to build the
        // shape. We do this by creating a Float32Array from the
        // JavaScript array, then use it to fill the current buffer.
        let positions: [f64; 8] = [1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0];
        console_log!(
            "rs (init_buffers): positions = {:?}",
            &Float64Array::from(positions.as_slice()).to_string()
        );

        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &Float64Array::from(positions.as_slice()),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        vec![NamedBuffer {
            name: "position",
            buffer: position_buffer,
        }]
    }

    pub fn new(width: u32, height: u32, ctx: WebGl2RenderingContext) -> CanvasSource {
        set_panic_hook();

        let shader_program = CanvasSource::init_shader_program(&ctx);
        let buffers = CanvasSource::init_buffers(&ctx);

        ctx.clear_color(0.0, 0.0, 0.0, 1.0); // Clear to black
        ctx.clear_depth(1.0); // Clear everything
        ctx.enable(WebGl2RenderingContext::DEPTH_TEST); // Enable depth testing
        ctx.depth_func(WebGl2RenderingContext::LEQUAL); // Near things obscure far things

        // Clear the canvas before we start drawing on it.
        ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        // Create a perspective matrix, a special matrix that is
        // used to simulate the distortion of perspective in a camera.
        // Our field of view is 45 degrees, with a width/height
        // ratio that matches the display size of the canvas
        // and we only want to see objects between 0.1 units
        // and 100 units away from the camera.

        let field_of_view = (45.0 * PI) / 180.0; // in radians
        let aspect_ratio = width as f32 / height as f32;
        let z_near = 0.01;
        let z_far = 100.0;

        let mut projection_matrix: Mat4 = {
			let mut matrix = [0.; 16];
			mat4::identity(&mut matrix);
			matrix
		};
		
        mat4::perspective(
            &mut projection_matrix,
            field_of_view,
            aspect_ratio,
            z_near,
            Some(z_far),
        );

		console_log!("rs (run): projection_matrix: {:?}", projection_matrix);
        // Set the drawing position to the "identity" point, which is
        // the center of the scene.

        let mut model_view_matrix: Mat4 = {
			let mut matrix = [0.; 16];
			mat4::identity(&mut matrix);
			matrix
		};
		
        // Now move the drawing position a bit to where we want to
        // start drawing the square
		{
			let ref_matrix = &model_view_matrix.clone();
			mat4::translate(&mut model_view_matrix, ref_matrix, &[-0.0, 0.0, -6.0]);
		}
        
		console_log!("rs (run): model_view_matrix: {:?}", model_view_matrix);
        // Tell WebGL how to pull out the positions from the position
        // buffer into the vertexPosition attribute.

        {
            let num_components = 2;
            let data_type = WebGl2RenderingContext::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;
			
			console_log!("rs (new): position buffer: {:?}", buffers.find("position").unwrap().to_string());

            ctx.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                buffers.find("position"),
            );

            ctx.vertex_attrib_pointer_with_i32(
                shader_program.attrib_locations.vertex_position.try_into().unwrap(),
                num_components,
                data_type,
                normalize,
                stride,
                offset,
            );

            ctx.enable_vertex_attrib_array(shader_program.attrib_locations.vertex_position.try_into().unwrap());
        }

        // Tell WebGL to use our program when drawing
        ctx.use_program(Some(&shader_program.program));
		

        // Set the shader uniforms
        ctx.uniform_matrix4fv_with_f32_array(
            Some(&shader_program.uniform_locations.projection_matrix),
            false,
            &projection_matrix,
        );

        ctx.uniform_matrix4fv_with_f32_array(
            Some(&shader_program.uniform_locations.model_view_matrix),
            false,
            &model_view_matrix,
        );

        let offset = 0;
        let vertex_count = 4;
        ctx.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, offset, vertex_count);
		console_log!("rs new(): prgm log: {:?}", ctx.get_program_info_log(&shader_program.program));
        console_log!("rs new(): successfully initialized source");
        CanvasSource { width, height, ctx }
    }
}
