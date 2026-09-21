use blitzkit::geometry::quad::Quad;

pub struct Ball {
    pub quad: Quad,
    pub velocity: glam::Vec2,
    pub visible: bool,
}

impl Ball {
    pub fn new(position: glam::Vec2, size: f32) -> Ball {
        Ball {
            quad: Quad::new(position, (size, size).into()),
            velocity: (0.0, 0.0).into(),
            visible: true,
        }
    }

    pub fn position(&self) -> glam::Vec2 {
        self.quad.position
    }

    pub fn radius(&self) -> f32 {
        self.quad.size.x * 0.5
    }

    pub fn update_position(&mut self, position: glam::Vec2) {
        self.quad = Quad::new(position, self.quad.size);
    }
}
