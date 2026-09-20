use blitkit::geometry::quad::Quad;

pub struct Ball {
    pub quad: Quad,
    pub velocity: cgmath::Vector2<f32>,
    pub visible: bool,
}

impl Ball {
    pub fn new(position: cgmath::Vector2<f32>, size: f32) -> Ball {
        Ball {
            quad: Quad::new(position, (size, size).into()),
            velocity: (0.0, 0.0).into(),
            visible: true,
        }
    }

    pub fn position(&self) -> cgmath::Vector2<f32> {
        self.quad.position
    }

    pub fn radius(&self) -> f32 {
        self.quad.size.x * 0.5
    }

    pub fn update_position(&mut self, position: cgmath::Vector2<f32>) {
        self.quad = Quad::new(position, self.quad.size);
    }
}
