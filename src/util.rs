#![macro_use]

use crate::ball::Ball;
use crate::player::Player;

// Speeds scale with the window so the game plays the same at any size.
/// Paddle speed in window heights per second.
pub const PLAYER_SPEED: f32 = 1.5;
/// Ball speed in window widths per second.
pub const BALL_SPEED: f32 = 0.75;

const BOUNCE_ANGLE: f32 = std::f32::consts::FRAC_PI_2;

/// The ball's velocity after bouncing off `player`, `speed` pixels per second away from it.
pub fn calc_ball_velocity(ball: &Ball, player: &Player, speed: f32) -> cgmath::Vector2<f32> {
    let diff_y = ball.position().y - player.position().y;
    let ratio = diff_y / player.size().y * 0.5;
    cgmath::Vector2 {
        x: (BOUNCE_ANGLE * ratio).cos() * (ball.position().x - player.position().x).signum(),
        y: (BOUNCE_ANGLE * ratio).sin(),
    } * speed
}

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}
