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
pub fn calc_ball_velocity(ball: &Ball, player: &Player, speed: f32) -> glam::Vec2 {
    let diff_y = ball.position().y - player.position().y;
    let ratio = diff_y / player.size().y * 0.5;
    glam::vec2(
        (BOUNCE_ANGLE * ratio).cos() * (ball.position().x - player.position().x).signum(),
        (BOUNCE_ANGLE * ratio).sin(),
    ) * speed
}

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bounce_angle_depends_on_where_it_hits() {
    // the left paddle, 120 tall, centered at y 300
    let player = Player::new((80.0, 300.0).into(), (20.0, 120.0).into());
    let centered = Ball::new((100.0, 300.0).into(), 20.0);
    let near_the_end = Ball::new((100.0, 260.0).into(), 20.0);

    let straight = calc_ball_velocity(&centered, &player, 600.0);
    let angled = calc_ball_velocity(&near_the_end, &player, 600.0);

    // both leave to the right, away from the paddle
    assert!(straight.x > 0.0);
    assert!(angled.x > 0.0);
    // a hit away from the center leaves at more of an angle
    assert!(angled.y.abs() > straight.y.abs());
  }
}
