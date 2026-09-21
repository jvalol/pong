use crate::ball::Ball;
use blitkit::geometry::quad::Quad;

pub struct Player {
    pub quad: Quad,
    pub score: u32,
    pub visible: bool,
}

impl Player {
    pub fn new(position: glam::Vec2, size: glam::Vec2) -> Player {
        Player {
            quad: Quad::new(position, size),
            score: 0,
            visible: false,
        }
    }

    pub fn position(&self) -> glam::Vec2 {
        self.quad.position
    }

    pub fn size(&self) -> glam::Vec2 {
        self.quad.size
    }

    pub fn update_y_position(&mut self, position: f32) {
        let position = (self.position().x, position);
        self.update_position(position.into());
    }

    pub fn update_position(&mut self, position: glam::Vec2) {
        self.quad = Quad::new(position, self.quad.size);
    }

    pub fn contains(&self, ball: &Ball) -> bool {
        let radii = self.size() * 0.5;
        let min = self.position() - radii;
        let max = self.position() + radii;

        let b_radii = glam::Vec2::splat(ball.radius());
        let b_min = ball.position() - b_radii;
        let b_max = ball.position() + b_radii;

        min.x < b_max.x && max.x > b_min.x && min.y < b_max.y && max.y > b_min.y
    }
}
