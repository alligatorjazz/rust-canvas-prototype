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
#[wasm_bindgen]
pub struct CanvasSource {
    width: u32,
    height: u32,
    data: Vec<u8>,
    ctx: WebGl2RenderingContext,
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

    fn compile_shader(
        ctx: &WebGl2RenderingContext,
        shader_type: u32,
        shader_source: &str,
    ) -> Result<WebGlShader, String> {
        console_log!("{:?}", ctx);
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
        let position_buffer = ctx.create_buffer();

        // Select the position_buffer as the one to apply buffer
        // operations to from here out.
        ctx.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            // as_ref() allows a borrow for a nested value
            position_buffer.as_ref(),
        );

        // Now pass the list of positions into WebGL to build the
        // shape. We do this by creating a Float32Array from the
        // JavaScript array, then use it to fill the current buffer.

        // TODO: find out why this conversion isn't working
        let raw_points = [1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0];
        let positions = Float64Array::new(&JsValue::from(raw_points.len()));

        (0..raw_points.len())
            .for_each(|i| positions.set_index(i.try_into().unwrap(), raw_points[i]));

        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        vec![NamedBuffer {
            name: "position",
            buffer: position_buffer.expect("Could not initialize position buffer."),
        }]
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

        let shader_program = CanvasSource::init_shader_program(&ctx);
        let buffers = CanvasSource::init_buffers(&ctx);

        // function drawScene(gl, programInfo, buffers) {
        //   gl.clearColor(0.0, 0.0, 0.0, 1.0); // Clear to black, fully opaque
        //   gl.clearDepth(1.0); // Clear everything
        //   gl.enable(gl.DEPTH_TEST); // Enable depth testing
        //   gl.depthFunc(gl.LEQUAL); // Near things obscure far things
        ctx.clear_color(0.0, 0.0, 0.0, 1.0); // Clear to black
        ctx.clear_depth(1.0); // Clear everything
        ctx.enable(WebGl2RenderingContext::DEPTH_TEST);
        ctx.depth_func(WebGl2RenderingContext::LEQUAL);

        // Clear the canvas before we start drawing on it.
        ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        //   gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        // Create a perspective matrix, a special matrix that is
        // used to simulate the distortion of perspective in a camera.
        // Our field of view is 45 degrees, with a width/height
        // ratio that matches the display size of the canvas
        // and we only want to see objects between 0.1 units
        // and 100 units away from the camera.

        //   const fieldOfView = (45 * Math.PI) / 180; // in radians
        //   const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
        //   const zNear = 0.1;
        //   const zFar = 100.0;
        //   const projectionMatrix = mat4.create();

        let field_of_view = (45.0 * PI) / 180.0; // in radians
        let aspect_ratio = width as f32 / height as f32;
        let z_near = 0.1;
        let z_far = 100.0;
        let mut projection_matrix: Mat4 = [0.; 16];

        // note: glmatrix.js always has the first argument
        // as the destination to receive the result.
        //   mat4.perspective(projectionMatrix, fieldOfView, aspect, zNear, zFar);
        mat4::perspective(
            &mut projection_matrix,
            field_of_view,
            aspect_ratio,
            z_near,
            Some(z_far),
        );

        //   // Set the drawing position to the "identity" point, which is
        //   // the center of the scene.
        //   const modelViewMatrix = mat4.create();
        let mut model_view_matrix: Mat4 = [0.; 16];
		mat4::translate(&mut model_view_matrix, &[0.; 16], &[-0.0, 0.0, -6.0]);
        //   // Now move the drawing position a bit to where we want to
        //   // start drawing the square.

        //   mat4.translate(
        //     modelViewMatrix, // destination matrix
        //     modelViewMatrix, // matrix to translate
        //     [-0.0, 0.0, -6.0]
        //   ); // amount to translate


        //   // Tell WebGL how to pull out the positions from the position
        //   // buffer into the vertexPosition attribute.
        //   {
        //     const numComponents = 2; // pull out 2 values per iteration
        //     const type = gl.FLOAT; // the data in the buffer is 32bit floats
        //     const normalize = false; // don't normalize
        //     const stride = 0; // how many bytes to get from one set of values to the next
        //     // 0 = use type and numComponents above
        //     const offset = 0; // how many bytes inside the buffer to start from
        //     gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
        //     gl.vertexAttribPointer(
        //       programInfo.attribLocations.vertexPosition,
        //       numComponents,
        //       type,
        //       normalize,
        //       stride,
        //       offset
        //     );
        //     gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition);
        //   }
		{
			let num_components = 2;
			let data_type = WebGl2RenderingContext::FLOAT;
			let normalize = false;
			let stride = 0;
			let offset = 9;

			ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffers.find("position"));
			ctx.vertex_attrib_pointer_with_i32(
				shader_program.attrib_locations.vertex_position as u32,
				num_components,
				data_type,
				normalize,
				stride,
				offset
			)
		}
		
		// TODO: finish implementing prgrm

        //   // Tell WebGL to use our program when drawing

        //   gl.useProgram(programInfo.program);

        //   // Set the shader uniforms

        //   gl.uniformMatrix4fv(
        //     programInfo.uniformLocations.projectionMatrix,
        //     false,
        //     projectionMatrix
        //   );
        //   gl.uniformMatrix4fv(
        //     programInfo.uniformLocations.modelViewMatrix,
        //     false,
        //     modelViewMatrix
        //   );

        //   {
        //     const offset = 0;
        //     const vertexCount = 4;
        //     gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
        //   }
        // }

        console_log!("rs new(): successfully initialized source");
        CanvasSource {
            width,
            height,
            data,
            ctx,
        }
    }
}
