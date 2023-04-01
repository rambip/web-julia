use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

mod drawer;
use drawer::{Drawer};

mod complex;
use complex::{Complex, Mandelbrot, Julia};


/*         _                                                  
  ___ ___ | | ___  _ __   _ __   __ _ _ __ __ _ _ __ ___  ___ 
 / __/ _ \| |/ _ \| '__| | '_ \ / _` | '__/ _` | '_ ` _ \/ __|
| (_| (_) | | (_) | |    | |_) | (_| | | | (_| | | | | | \__ \
 \___\___/|_|\___/|_|    | .__/ \__,_|_|  \__,_|_| |_| |_|___/
                         |_|                                  
*/

const AFFINE_RED : (f32, f32)  = (300.0,  0.0);   // affine coefficient for red
const AFFINE_GREE : (f32, f32) = (250.0, -20.0);   // affine coefficient for green
const AFFINE_BLUE : (f32, f32) = (300.0, -10.0);   // affine coefficient for blue
const AFFINE_ALPH : (f32, f32) = (1000.0, -50.0);   // affine coefficient for alpha

#[wasm_bindgen]
pub struct MandelbrotDrawer {
    drawer: Drawer,
    mandel: Mandelbrot,
    size: usize,
    xmin: f32,
    ymin: f32,
    scale: f32,
}

#[wasm_bindgen]
impl MandelbrotDrawer {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize, depth: usize, 
               xmin: f32, ymin: f32, scale: f32, 
               ctx: CanvasRenderingContext2d) -> Self {

        let drawer = Drawer::new(size, size, ctx);
        let mandel = Mandelbrot {depth};
        Self {drawer, mandel, size, xmin, ymin, scale}
    }
    
    #[wasm_bindgen]
    pub fn display(&mut self) -> Result<(), JsValue> {
        let (mandel, xmin, ymin, size, scale) = (
            &self.mandel, self.xmin, self.ymin, self.size, self.scale);

        // color of point on the complex plane
        let color = |d: f32| (
            (AFFINE_RED.0  * d + AFFINE_RED.1 ) as u8,
            (AFFINE_GREE.0 * d + AFFINE_GREE.1) as u8,
            (AFFINE_BLUE.0 * d + AFFINE_BLUE.1) as u8,
            (AFFINE_ALPH.0 * d + AFFINE_ALPH.1) as u8,
        );

        // pixel space to complex plane
        let map_coord = |x: usize, y:usize|
            Complex {a: xmin + x as f32 * scale / size  as f32,
                     b: ymin + y as f32 * scale / size as f32};

        self.drawer.generate(|x, y| color(mandel.gradient(map_coord(x, y))));

        self.drawer.display()
    }
}




#[wasm_bindgen]
pub struct JuliaDrawer {
    drawer: Drawer,
    julia: Julia,
    size: usize,
    xmin: f32,
    ymin: f32,
    scale: f32,
}

#[wasm_bindgen]
impl JuliaDrawer {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize, depth: usize, 
               xmin: f32, ymin: f32, scale: f32, 
               ctx: CanvasRenderingContext2d) -> Self {

        let drawer = Drawer::new(size, size, ctx);
        let julia = Julia {c: Complex {a: 0.0, b: 0.0}, depth};
        Self {drawer, julia, size, xmin, ymin, scale}
    }

    #[wasm_bindgen]
    pub fn set_complex(&mut self, a: f32, b: f32) {
        self.julia.c = Complex {a, b};
    }
    
    #[wasm_bindgen]
    pub fn display(&mut self) -> Result<(), JsValue> {
        let (julia, xmin, ymin, size, scale) = (
            &self.julia, self.xmin, self.ymin, self.size, self.scale);

        // color of point on the complex plane
        let color = |d: f32| (
            (AFFINE_RED.0  * d + AFFINE_RED.1 ) as u8,
            (AFFINE_GREE.0 * d + AFFINE_GREE.1) as u8,
            (AFFINE_BLUE.0 * d + AFFINE_BLUE.1) as u8,
            (AFFINE_ALPH.0 * d + AFFINE_ALPH.1) as u8,
        );

        // pixel space to complex plane
        let map_coord = |x: usize, y:usize|
            Complex {a: xmin + x as f32 * scale / size  as f32,
                     b: ymin + y as f32 * scale / size as f32};

        self.drawer.generate(|x, y| color(julia.gradient(map_coord(x, y))));
            

        self.drawer.display()
    }
}
