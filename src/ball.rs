#[derive(Debug, Clone)]
pub struct Ball {
    pub position: [f32; 2],
    pub direction: [f32; 2],
    pub radius: f32,
}

impl Ball {
    pub fn update(&mut self, step: f32) {
        let pos = self.move_position(step);

        let (pos, dir) = Self::check_collision_with_borders(pos, self.direction, self.radius);

        self.position = pos;
        self.direction = dir;
    }

    fn move_position(&self, step: f32) -> [f32; 2] {
        [self.position[0] + step*self.direction[0],
         self.position[1] + step*self.direction[1]]
    }

    fn check_collision_with_borders(pos: [f32; 2], dir: [f32; 2], r: f32) -> ([f32; 2], [f32; 2]) {
        if pos[0] > 1.0 && pos[1] > 1.0 {
            ([2.0-pos[0], 2.0-pos[1]], [-dir[0], -dir[1]])
        } else if pos[0] > 1.0 && pos[1] < 0.0 {
            ([2.0-pos[0], -pos[1]], [-dir[0], -dir[1]])
        } else if pos[0] < 0.0 && pos[1] < 0.0 {
            ([-pos[0], -pos[1]], [-dir[0], -dir[1]])
        } else if pos[0] < 0.0 && pos[1] > 1.0 {
            ([-pos[0], 2.0-pos[1]], [-dir[0], -dir[1]])
        } else if pos[0] < 0.0+r {
            ([2.0*r-pos[0], pos[1]], [-dir[0], dir[1]])
        } else if pos[0] > 1.0-r {
            ([2.0*(1.0-r)-pos[0], pos[1]], [-dir[0], dir[1]])
        } else if pos[1] < 0.0+r {
            ([pos[0], 2.0*r-pos[1]], [dir[0], -dir[1]])
        } else if pos[1] > 1.0-r {
            ([pos[0], 2.0*(1.0-r)-pos[1]], [dir[0], -dir[1]])
        } else {
            (pos, dir)
        }
    }
}
