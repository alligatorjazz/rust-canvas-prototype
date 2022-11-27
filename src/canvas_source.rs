#[cfg(web_sys_unstable_apis)]
use wasm_bindgen::prelude::*;

extern crate fixedbitset;
extern crate web_sys;


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
        let mut blood: Vec<u8> = vec![252, 3, 27, 255];
        let pixel = blood.clone();

        for _ in 0..self.width {
            for _ in 0..self.height {
                blood.extend(&pixel)
            }
        }

        self.data = blood;
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
