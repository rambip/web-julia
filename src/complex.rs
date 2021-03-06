#[derive(Clone)]
pub struct Complex {
    pub a : f64,
    pub b : f64,
}

impl Complex {
    pub fn add_to_it(&mut self, c : &Complex){
        self.a += c.a;
        self.b += c.b;
    }

    pub fn square_it(&mut self){
        let a = self.a*self.a - self.b*self.b;
        let b = 2f64*self.a*self.b;
        self.a = a;
        self.b = b;
    }
    
    pub fn scale(&self, r: f64) -> Self {
        Complex {a:self.a*r, b:self.b*r}
    }
}


pub struct Mandelbrot {
    pub depth: usize,
}

impl Mandelbrot {
    pub fn calc_depth(&self, c: Complex) -> usize{
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

}




pub struct Julia {
    pub c : Complex,
    pub depth: usize,
}

impl Julia {
    pub fn calc_depth(&self, mut z: Complex) -> usize {
        for i in 0..self.depth {
            z.add_to_it(&self.c);
            z.square_it();
            if z.a*z.a+z.b*z.b > 5f64{
                return i;
            }
        }
        return self.depth+1;
    }
}
