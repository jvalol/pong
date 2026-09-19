#![macro_use]

use crate::ball::Ball;
use crate::player::Player;

// Units per second. The playfield is 2.0 units across.
pub const PLAYER_SPEED: f32 = 3.0;
pub const BALL_SPEED: f32 = 1.5;

const BOUNCE_ANGLE: f32 = std::f32::consts::FRAC_PI_2;

pub fn calc_ball_velocity(ball: &Ball, player: &Player) -> cgmath::Vector2<f32> {
    let diff_y = ball.position().y - player.position().y;
    let ratio = diff_y / player.size().y * 0.5;
    cgmath::Vector2 {
        x: (BOUNCE_ANGLE * ratio).cos() * -player.position().x.signum(),
        y: (BOUNCE_ANGLE * ratio).sin(),
    } * BALL_SPEED
}

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}
