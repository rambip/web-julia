#[derive(Clone)]
pub struct Complex {
    pub a : f32,
    pub b : f32,
}

impl Complex {
    pub fn add_to_it(&mut self, c : &Complex){
        self.a += c.a;
        self.b += c.b;
    }

    pub fn square_it(&mut self){
        let a = self.a*self.a - self.b*self.b;
        let b = 2f32*self.a*self.b;
        self.a = a;
        self.b = b;
    }
}


pub struct Mandelbrot {
    pub depth: usize,
}

impl Mandelbrot {
    pub fn gradient(&self, c: Complex) -> f32{
        let mut z = Complex{a:0f32, b:0f32};
        for i in 0..self.depth {
            z.square_it();
            z.add_to_it(&c);
            if z.a*z.a + z.b*z.b > 5f32 {
                return i as f32 / self.depth as f32
            }
        }
        return 1.0
    }

}




pub struct Julia {
    pub c : Complex,
    pub depth: usize,
}

impl Julia {
    pub fn gradient(&self, mut z: Complex) -> f32 {
        for i in 0..self.depth {
            z.square_it();
            z.add_to_it(&self.c);
            if z.a*z.a+z.b*z.b > 5f32{
                return i as f32 / self.depth as f32 
            }
        }
        return 1.0
    }
}
