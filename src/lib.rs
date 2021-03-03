use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::Clamped;

#[derive(Clone)]
struct Complex {
    a : f64,
    b : f64,
}

impl Complex {
    fn add_to_it(&mut self, c : &Complex){
        self.a += c.a;
        self.b += c.b;
    }

    fn square_it(&mut self){
        let a = self.a*self.a - self.b*self.b;
        let b = 2f64*self.a*self.b;
        self.a = a;
        self.b = b;
    }
}


#[wasm_bindgen]
pub struct Mandelbrot {
    depth: usize
}

#[wasm_bindgen]
impl Mandelbrot {
    #[wasm_bindgen(constructor)]
    pub fn new(depth: usize) -> Mandelbrot{
        Mandelbrot {depth}
    }
    fn calc_depth(&self, c: Complex) -> usize{
        let mut z = Complex{a:0f64, b:0f64};
        for i in 0..self.depth {
            z.add_to_it(&c);
            z.square_it();
            if z.a*z.a + z.b*z.b > 5f64 {
                return i;
            }
        }
        return self.depth+1;
    }
    pub fn draw(&self, ctx: &CanvasRenderingContext2d, size_pixel : u32,
                xmin: f64, ymin: f64, size_plane: f64) -> Result<(), JsValue>{

        let mut data = Vec::new();

        for y_grid in 0..size_pixel {
            for x_grid in 0..size_pixel {
                let a = xmin + (x_grid as f64) / (size_pixel as f64) * size_plane;
                let b = ymin + (y_grid as f64) / (size_pixel as f64) * size_plane;
                
                // calculate depth (iterations before diverging)
                let d = self.calc_depth(Complex {a, b}) as f32;

                // push corresponding pixel:
                data.push((d*5.0) as u8); // red
                data.push((d*4.5) as u8); // green
                data.push((80.0+d*4.0) as u8); // blue
                data.push(((d-1.0)*15.0) as u8); // transparency
            }
        }
        // create imageData
        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), size_pixel, size_pixel)?;
        // draw on the screen
        ctx.put_image_data(&data, 0.0, 0.0)

    }

}

#[wasm_bindgen]
pub struct Julia{
    c : Complex,
    depth: usize
}

#[wasm_bindgen]
impl Julia {
    #[wasm_bindgen(constructor)]
    pub fn new(a :f64, b:f64, depth: usize) -> Julia{
        let c = Complex {a, b};

        Julia {c, depth}
    }

    fn calc_depth(&self, mut z: Complex) -> usize {
        for i in 0..self.depth {
            z.add_to_it(&self.c);
            z.square_it();
            if z.a*z.a+z.b*z.b > 5f64{
                return i;
            }
        }
        return self.depth+1;
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, size_pixel: u32,
                xmin: f64, ymin: f64, size_plane : f64) -> Result<(), JsValue>{

        let mut data = Vec::new();

        for y_grid in 0..size_pixel {
            for x_grid in 0..size_pixel{
                let a = xmin + (x_grid as f64) / (size_pixel as f64) * size_plane;
                let b = ymin + (y_grid as f64) / (size_pixel as f64) * size_plane;

                // calculate depth at this point (iterations before diverging)
                let d = self.calc_depth(Complex {a, b}) as f32;

                // push corresponding pixel
                data.push((d*5.0) as u8); // red
                data.push((d*4.5) as u8);  // green
                data.push((80.0+d*4.0) as u8); // blue
                data.push(((d-1.0)*15.0) as u8); // transparency
            }
        }
        // create Image Data
        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), size_pixel, size_pixel)?;
        // draw on the screen
        ctx.put_image_data(&data, 0.0, 0.0)

    }
}
