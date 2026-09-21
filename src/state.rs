use crate::ball::Ball;
use crate::player::Player;
use blitkit::geometry::Geometry;
use blitkit::renderer::render_text::{RenderText, TextRenderer, UNBOUNDED_F32};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
  MainMenu,
  Serving,
  Playing,
  Paused,
  GameOver,
  Quitting,
}

pub struct PongText {
  pub render_text: RenderText,
  pub visible: bool,
}

impl PongText {
  pub fn focused(&self) -> bool {
    self.render_text.focused
  }

  pub fn set_focus(&mut self, focused: bool) {
    self.render_text.focused = focused;
  }
}

pub struct State {
  pub game_state: GameState,
  pub player1: Player,
  pub player2: Player,
  pub ball: Ball,
  pub title_text: PongText,
  pub play_button: PongText,
  pub quit_button: PongText,
  pub player1_score: PongText,
  pub player2_score: PongText,
  pub win_text: PongText,
  /// Seconds since the previous update.
  pub delta_time: f32,
  /// Window size in pixels. Everything in the game is laid out relative to it.
  pub field: glam::Vec2,
}

impl State {
  pub fn new() -> Self {
    Self {
      game_state: GameState::MainMenu,
      delta_time: 0.0,
      // sized and placed by layout()
      player1: Player::new((0.0, 0.0).into(), (0.0, 0.0).into()),
      player2: Player::new((0.0, 0.0).into(), (0.0, 0.0).into()),
      ball: Ball::new((0.0, 0.0).into(), 0.0),
      title_text: PongText {
        visible: false,
        render_text: RenderText {
          position: (20.0, 20.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("PONG"),
          size: 64.0,
          ..Default::default()
        },
      },
      play_button: PongText {
        visible: false,
        render_text: RenderText {
          position: (40.0, 100.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("Play"),
          size: 32.0,
          ..Default::default()
        },
      },
      quit_button: PongText {
        visible: false,
        render_text: RenderText {
          position: (40.0, 160.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("Quit"),
          size: 32.0,
          ..Default::default()
        },
      },
      player1_score: PongText {
        visible: false,
        render_text: RenderText {
          // position: (render.width() * 0.25, 20.0).into(),
          position: (20.0, 20.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("0"),
          size: 32.0,
          ..Default::default()
        },
      },
      player2_score: PongText {
        visible: false,
        render_text: RenderText {
          // position: (render.width() * 0.75, 20.0).into(),
          position: (120.0, 20.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("0"),
          size: 32.0,
          ..Default::default()
        },
      },
      win_text: PongText {
        visible: false,
        render_text: RenderText {
          // centered in the window by layout()
          position: (0.0, 0.0).into(),
          bounds: (UNBOUNDED_F32, UNBOUNDED_F32).into(),
          size: 32.0,
          centered: true,
          ..Default::default()
        },
      },
      field: (0.0, 0.0).into(),
    }
  }

  /// Sizes and places the paddles, ball, and win text for a window of `size` pixels.
  /// When the window changes size mid-game, positions and the ball's velocity scale with it.
  pub fn layout(&mut self, size: glam::Vec2) {
    let old = self.field;
    self.field = size;

    let paddle_size = glam::vec2(size.x * 0.025, size.y * 0.2);
    self.player1.quad.size = paddle_size;
    self.player2.quad.size = paddle_size;
    self.ball.quad.size = (size.x * 0.025, size.x * 0.025).into();

    if old.x > 0.0 && old.y > 0.0 {
      let scale = |v: glam::Vec2| -> glam::Vec2 {
        (v.x * size.x / old.x, v.y * size.y / old.y).into()
      };
      self.player1.update_y_position(self.player1.position().y * size.y / old.y);
      self.player2.update_y_position(self.player2.position().y * size.y / old.y);
      self.ball.update_position(scale(self.ball.position()));
      self.ball.velocity = scale(self.ball.velocity);
    } else {
      self.player1.update_y_position(size.y * 0.5);
      self.player2.update_y_position(size.y * 0.5);
      self.ball.update_position(size * 0.5);
    }
    self.player1.update_position((size.x * 0.1, self.player1.position().y).into());
    self.player2.update_position((size.x * 0.9, self.player2.position().y).into());

    self.win_text.render_text.position = size * 0.5;
  }

  pub fn initialize(&mut self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
    self.update_geometry(geometry);
    self.update_text(text_renderer);
  }

  pub fn update(&self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
    self.update_geometry(geometry);
    self.update_text(text_renderer);
  }

  fn update_geometry(&self, geometry: &mut Geometry) {
    if self.player1.visible {
      geometry.push_quad(&self.player1.quad);
    }

    if self.player2.visible {
      geometry.push_quad(&self.player2.quad);
    }

    if self.ball.visible {
      geometry.push_quad(&self.ball.quad);
    }
  }

  fn update_text(&self, text_renderer: &mut TextRenderer) {
    for text in [
      &self.title_text,
      &self.play_button,
      &self.quit_button,
      &self.player1_score,
      &self.player2_score,
      &self.win_text,
    ]
    .iter()
    {
      if text.visible {
        text_renderer.push_render_text(text.render_text.clone());
      }
    }
  }

  pub fn pause_game(&mut self) {
    if self.game_state == GameState::Playing {
      self.game_state = GameState::Paused;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn state_of(width: f32, height: f32) -> State {
    let mut state = State::new();
    state.layout((width, height).into());
    state
  }

  #[test]
  fn layout_sizes_pieces_from_the_window() {
    let state = state_of(800.0, 600.0);

    assert_eq!(state.player1.size().x, 20.0);
    assert_eq!(state.player1.size().y, 120.0);
    assert_eq!(state.player2.size(), state.player1.size());
    assert_eq!(state.ball.quad.size.x, 20.0);
  }

  #[test]
  fn ball_is_square_in_a_wide_window() {
    let state = state_of(1600.0, 400.0);

    assert_eq!(state.ball.quad.size.x, state.ball.quad.size.y);
  }

  #[test]
  fn paddles_sit_inside_each_edge() {
    let state = state_of(800.0, 600.0);

    assert_eq!(state.player1.position().x, 80.0);
    assert_eq!(state.player2.position().x, 720.0);
  }

  #[test]
  fn first_layout_centers_everything() {
    let state = state_of(800.0, 600.0);

    assert_eq!(state.ball.position().x, 400.0);
    assert_eq!(state.ball.position().y, 300.0);
    assert_eq!(state.player1.position().y, 300.0);
    assert_eq!(state.player2.position().y, 300.0);
  }

  #[test]
  fn resize_scales_positions() {
    let mut state = state_of(800.0, 600.0);
    state.ball.update_position((200.0, 150.0).into());
    state.ball.velocity = (100.0, 50.0).into();

    state.layout((1600.0, 1200.0).into());

    assert_eq!(state.ball.position().x, 400.0);
    assert_eq!(state.ball.position().y, 300.0);
    assert_eq!(state.ball.velocity.x, 200.0);
    assert_eq!(state.ball.velocity.y, 100.0);
  }

  #[test]
  fn layout_centers_win_text() {
    let mut state = state_of(800.0, 600.0);
    assert_eq!(state.win_text.render_text.position.x, 400.0);

    state.layout((1000.0, 500.0).into());

    assert_eq!(state.win_text.render_text.position.x, 500.0);
    assert_eq!(state.win_text.render_text.position.y, 250.0);
  }
}
