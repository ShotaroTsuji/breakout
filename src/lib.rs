#[derive(Debug, Clone)]
pub struct BoardSize {
    pub width: usize,
    pub height: usize,
}

impl glium::uniforms::AsUniformValue for BoardSize {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        let v = [self.width as f32, self.height as f32];
        glium::uniforms::UniformValue::Vec2(v)
    }
}

#[derive(Debug, Clone)]
pub struct Ball {
    pub position: [f32; 2],
    pub direction: [f32; 2],
    pub radius: f32,
}

impl Ball {
    pub fn update(&mut self, step: f32) {
        let new_pos =
            [self.position[0] + step * self.direction[0],
             self.position[1] + step * self.direction[1]];

        if ( new_pos[0] > 1.0 && new_pos[1] > 1.0 ) ||
           ( new_pos[0] < 0.0 && new_pos[1] < 0.0 ) ||
           ( new_pos[0] > 1.0 && new_pos[1] < 0.0 ) ||
           ( new_pos[0] < 0.0 && new_pos[1] > 1.0 ) {
            self.direction[0] *= -1.0;
            self.direction[1] *= -1.0;
        } else if new_pos[0] > 1.0 || new_pos[0] < 0.0 {
            self.direction[0] *= -1.0;
        } else if new_pos[1] > 1.0 || new_pos[1] < 0.0 {
            self.direction[1] *= -1.0;
        }

        self.position = new_pos;
    }
}

#[derive(Debug)]
pub struct Breakout {
    pub ball: Ball,
    pub board: BoardSize,
}

impl Breakout {
    pub fn update(&mut self, step: f32) {
        self.ball.update(step);
    }
}
