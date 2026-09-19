use crate::ball::Ball;
use crate::player::Player;
use dynamo_lib::geometry::Geometry;
use dynamo_lib::renderer::render_text::{RenderText, TextRenderer, UNBOUNDED_F32};

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
  pub field: cgmath::Vector2<f32>,
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
  pub fn layout(&mut self, size: cgmath::Vector2<f32>) {
    let old = self.field;
    self.field = size;

    let paddle_size = cgmath::Vector2::new(size.x * 0.025, size.y * 0.2);
    self.player1.quad.size = paddle_size;
    self.player2.quad.size = paddle_size;
    self.ball.quad.size = (size.x * 0.025, size.x * 0.025).into();

    if old.x > 0.0 && old.y > 0.0 {
      let scale = |v: cgmath::Vector2<f32>| -> cgmath::Vector2<f32> {
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
    for text in vec![
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
