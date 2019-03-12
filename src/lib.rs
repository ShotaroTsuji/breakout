pub mod ball;

use crate::ball::Ball;

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
