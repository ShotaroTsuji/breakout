use glium::implement_vertex;

#[derive(Debug)]
pub struct Bricks {
    num_vertical: usize,
    num_horizontal: usize,
    lifes: Vec<usize>,
}

#[derive(Debug,Clone,Copy)]
pub struct BrickVertex {
    position: [f32; 2],
    life: i32,
}

implement_vertex!(BrickVertex, position, life);

impl Bricks {
    pub fn new(num_h: usize, num_v: usize) -> Self {
        Bricks {
            num_vertical: num_v,
            num_horizontal: num_h,
            lifes: vec![0; num_v*num_h],
        }
    }

    pub fn new_with<F>(num_h: usize, num_v: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> usize,
    {
        let mut lifes = Vec::with_capacity(num_v*num_h);

        for y in 0..num_v {
            for x in 0..num_h {
                lifes.push(f(x, y));
            }
        }

        Bricks {
            num_vertical: num_v,
            num_horizontal: num_h,
            lifes: lifes,
        }
    }

    pub fn get_life(&self, x: usize, y: usize) -> Option<usize> {
        self.lifes.get(x + y*self.num_horizontal).cloned()
    }

    pub fn to_vertices(&self) -> (Vec<BrickVertex>, Vec<u16>) {
        let mut vec = Vec::with_capacity(self.lifes.len()*4);
        let mut indices = Vec::new();
        let mut index = 0;

        let n = self.num_horizontal as f32;
        let m = self.num_vertical as f32;

        for x in 0..self.num_horizontal {
            for y in 0..self.num_vertical {
                let xf = x as f32;
                let yf = y as f32;

                vec.push(BrickVertex {
                    position: [xf/n, yf/m],
                    life: self.get_life(x, y).unwrap() as i32,
                });
                vec.push(BrickVertex {
                    position: [(xf+1.0)/n, yf/m],
                    life: self.get_life(x, y).unwrap() as i32,
                });
                vec.push(BrickVertex {
                    position: [(xf+1.0)/n, (yf+1.0)/m],
                    life: self.get_life(x, y).unwrap() as i32,
                });
                vec.push(BrickVertex {
                    position: [xf/n, (yf+1.0)/m],
                    life: self.get_life(x, y).unwrap() as i32,
                });

                indices.push(index);
                indices.push(index+3);
                indices.push(index+1);

                indices.push(index+3);
                indices.push(index+2);
                indices.push(index+1);

                index += 4;
            }
        }

        (vec, indices)
    }
}
