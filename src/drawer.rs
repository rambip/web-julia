use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};


pub struct Drawer {
    pub width: usize,
    pub height: usize,
    pub frame: u32,
    raw_pixels: Vec<u8>,
    ctx: CanvasRenderingContext2d,
}


pub type Color = (u8, u8, u8, u8);


impl Drawer {
    pub fn new(width: usize, height: usize, ctx: CanvasRenderingContext2d) -> Drawer {
        let raw_pixels = vec![0u8; width*height*4];
        Drawer {width, height, frame:0, raw_pixels, ctx}
    }

    pub fn generate(&mut self, f: impl Fn(usize, usize) -> Color) {
        let mut ix = 0usize;
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b, a) =  f(x, y);

                self.raw_pixels[ix    ] = r;
                self.raw_pixels[ix + 1] = g;
                self.raw_pixels[ix + 2] = b;
                self.raw_pixels[ix + 3] = a;
                ix += 4;
            }
        }
    }

    pub fn display(&self) -> Result<(), JsValue> {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.raw_pixels[..]), 
            self.width as u32,
            self.height as u32)?;

        self.ctx.put_image_data(&data, 0.0, 0.0)
    }
}
