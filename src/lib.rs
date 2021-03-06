use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

mod drawer;
use drawer::{Drawer};

mod complex;
use complex::{Complex, Mandelbrot, Julia};


#[wasm_bindgen]
pub struct MandelbrotDrawer {
    drawer: Drawer,
    mandel: Mandelbrot,
    size: usize,
    xmin: f64,
    ymin: f64,
    scale: f64,
}

#[wasm_bindgen]
impl MandelbrotDrawer {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize, depth: usize, 
               xmin: f64, ymin: f64, scale: f64, 
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
        let color = |d: u8| (d *5, d*4, 80+d*40, (d-1)*15);

        // pixel space to complex plane
        let map_coord = |x: usize, y:usize|
            Complex {a: xmin + x as f64,
                     b: ymin + y as f64}
                    .scale(size as f64 * scale);

        self.drawer.generate(|x, y| color(mandel.calc_depth(map_coord(x, y)) as u8));
        self.drawer.display()
    }
}




#[wasm_bindgen]
pub struct JuliaDrawer {
    drawer: Drawer,
    julia: Julia,
    size: usize,
    xmin: f64,
    ymin: f64,
    scale: f64,
}

#[wasm_bindgen]
impl JuliaDrawer {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize, depth: usize, 
               xmin: f64, ymin: f64, scale: f64, 
               ctx: CanvasRenderingContext2d) -> Self {

        let drawer = Drawer::new(size, size, ctx);
        let julia = Julia {c: Complex {a: 0.0, b: 0.0}, depth};
        Self {drawer, julia, size, xmin, ymin, scale}
    }

    pub fn set_complex(&mut self, a: f64, b: f64) {
        self.julia.c = Complex {a, b};
    }
    
    #[wasm_bindgen]
    pub fn display(&mut self) -> Result<(), JsValue> {
        let (julia, xmin, ymin, size, scale) = (
            &self.julia, self.xmin, self.ymin, self.size, self.scale);

        // color of point on the complex plane
        let color = |d: u8| (d *5, d*4, 80+d*40, (d-1)*15);

        // pixel space to complex plane
        let map_coord = |x: usize, y:usize|
            Complex {a: xmin + x as f64,
                     b: ymin + y as f64}
                    .scale(size as f64 * scale);

        self.drawer.generate(|x, y| color(julia.calc_depth(map_coord(x, y)) as u8));
        self.drawer.display()
    }
}
