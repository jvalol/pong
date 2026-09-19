use crate::any;
use crate::input::Input;
use crate::pong_game::*;
use crate::state::*;
use crate::player::Player;
use crate::util;

pub trait System {
  #[allow(unused_variables)]
  fn start(&mut self, game: &mut State) {}
  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>);
}

pub struct VisibilitySystem;
impl System for VisibilitySystem {
  fn update_state(&self, _input: &mut Input, state: &mut State, _events: &mut Vec<Event>) {
    let is_in_game = any!(
      state.game_state,
      GameState::Serving,
      GameState::Playing,
      GameState::GameOver
    );
    state.ball.visible = is_in_game && state.game_state != GameState::GameOver;
    state.player1.visible = is_in_game;
    state.player1_score.visible = is_in_game;
    state.player2.visible = is_in_game;
    state.player2_score.visible = is_in_game;

    state.title_text.visible =
      state.game_state == GameState::MainMenu || state.game_state == GameState::Paused;
    state.play_button.visible =
      state.game_state == GameState::MainMenu || state.game_state == GameState::Paused;
    state.quit_button.visible = state.game_state == GameState::MainMenu;

    state.win_text.visible = state.game_state == GameState::GameOver;
  }
}

#[derive(Debug)]
pub struct MenuSystem;

impl System for MenuSystem {
  fn start(&mut self, state: &mut State) {
    state.player1.score = 0;
    state.player2.score = 0;
    state.player1.update_y_position(state.field.y * 0.5);
    state.player2.update_y_position(state.field.y * 0.5);
    state.play_button.render_text.focused = true;
    state.quit_button.render_text.focused = false;
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Quitting;

      input.esc_pressed = false;
    }

    if state.play_button.focused() && input.ui_down_pressed() {
      events.push(Event::FocusChanged);
      state.play_button.set_focus(false);
      state.quit_button.set_focus(true);
    } else if state.quit_button.focused() && input.ui_up_pressed() {
      events.push(Event::FocusChanged);
      state.quit_button.set_focus(false);
      state.play_button.set_focus(true);
    }

    if state.play_button.focused() && input.enter_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Serving;
    } else if state.quit_button.focused() && input.enter_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Quitting;
    }
  }
}

#[derive(Debug)]
pub struct PlaySystem;

impl System for PlaySystem {
  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      input.clear();
      events.push(Event::ButtonPressed);
      state.game_state = GameState::MainMenu;

      input.esc_pressed = false;
    }

    let step = util::PLAYER_SPEED * state.field.y * state.delta_time;
    let field_height = state.field.y;
    move_player(
      &mut state.player1,
      input.p1_up_pressed,
      input.p1_down_pressed,
      step,
      field_height,
    );
    move_player(
      &mut state.player2,
      input.p2_up_pressed,
      input.p2_down_pressed,
      step,
      field_height,
    );
  }
}

/// Moves a paddle up or down by `step` pixels and keeps it inside the window.
fn move_player(player: &mut Player, up: bool, down: bool, step: f32, field_height: f32) {
  let mut y = player.position().y;
  if up {
    y -= step;
  }
  if down {
    y += step;
  }
  let half_height = player.size().y * 0.5;
  player.update_y_position(y.max(half_height).min(field_height - half_height));
}

#[derive(Debug)]
pub struct PauseSystem;

impl System for PauseSystem {
  fn start(&mut self, state: &mut State) {
    state.title_text.render_text.text = String::from("Paused");
    state.play_button.render_text.text = String::from("Resume");
    state.play_button.render_text.focused = true;
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if state.play_button.focused() && input.enter_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Playing;
      state.title_text.render_text.text = String::from("PONG");
      state.play_button.render_text.text = String::from("Play");
    }
  }
}

#[derive(Debug)]
pub struct BallSystem;

impl System for BallSystem {
  fn update_state(&self, _input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    // bounce the ball off the players
    if state.player1.contains(&state.ball) {
      events.push(Event::BallBounce(state.ball.position()));
      // move the ball out to the paddle's right face so it doesn't bounce again next frame
      let x = state.player1.position().x + state.player1.size().x * 0.5 + state.ball.radius();
      state.ball.update_position((x, state.ball.position().y).into());
      state.ball.velocity =
        util::calc_ball_velocity(&state.ball, &state.player1, util::BALL_SPEED * state.field.x);
    } else if state.player2.contains(&state.ball) {
      events.push(Event::BallBounce(state.ball.position()));
      // move the ball out to the paddle's left face so it doesn't bounce again next frame
      let x = state.player2.position().x - state.player2.size().x * 0.5 - state.ball.radius();
      state.ball.update_position((x, state.ball.position().y).into());
      state.ball.velocity =
        util::calc_ball_velocity(&state.ball, &state.player2, util::BALL_SPEED * state.field.x);
    }

    state
      .ball
      .update_position(state.ball.position() + state.ball.velocity * state.delta_time);
    let top = state.ball.radius();
    let bottom = state.field.y - state.ball.radius();
    if state.ball.position().y < top {
      events.push(Event::BallBounce(state.ball.position()));
      state.ball.update_position((state.ball.position().x, top).into());
      state.ball.velocity.y *= -1.0;
    } else if state.ball.position().y > bottom {
      events.push(Event::BallBounce(state.ball.position()));
      state.ball.update_position((state.ball.position().x, bottom).into());
      state.ball.velocity.y *= -1.0;
    }

    if state.ball.position().x > state.field.x {
      state.player1.score += 1;
      if state.player1.score >= 5 {
        state.game_state = GameState::GameOver;
      } else {
        state.game_state = GameState::Serving;
        events.push(Event::Score(0));
      }
    } else if state.ball.position().x < 0.0 {
      state.player2.score += 1;
      if state.player2.score >= 5 {
        state.game_state = GameState::GameOver;
      } else {
        state.game_state = GameState::Serving;
        events.push(Event::Score(1));
      }
    }
  }
}

pub struct ServingSystem {
  last_time: std::time::Instant,
}

impl ServingSystem {
  pub fn new() -> Self {
    Self {
      last_time: std::time::Instant::now(),
    }
  }
}

impl System for ServingSystem {
  fn start(&mut self, state: &mut State) {
    self.last_time = std::time::Instant::now();
    let direction = (state.ball.position().x - state.field.x * 0.5).signum();
    state.ball.update_position(state.field * 0.5);
    state.ball.velocity =
      cgmath::Vector2::unit_x() * direction * -util::BALL_SPEED * state.field.x;
    state.player1_score.render_text.text = format!("{}", state.player1.score);
    state.player2_score.render_text.text = format!("{}", state.player2.score);
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      input.clear();
      events.push(Event::ButtonPressed);
      state.game_state = GameState::MainMenu;

      input.esc_pressed = false;
    }

    let current_time = std::time::Instant::now();
    let delta_time = current_time - self.last_time;
    if delta_time.as_secs_f32() > 2.0 {
      state.game_state = GameState::Playing;
    }
  }
}

pub struct GameOverSystem {
  last_time: std::time::Instant,
}

impl GameOverSystem {
  pub fn new() -> Self {
    Self {
      last_time: std::time::Instant::now(),
    }
  }
}

impl System for GameOverSystem {
  fn start(&mut self, state: &mut State) {
    self.last_time = std::time::Instant::now();

    state.player1_score.render_text.text = format!("{}", state.player1.score);
    state.player2_score.render_text.text = format!("{}", state.player2.score);

    state.win_text.render_text.text = if state.player1.score > state.player2.score {
      String::from("Player 1 wins!")
    } else {
      String::from("Player 2 wins!")
    };
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Quitting;

      input.esc_pressed = false;
    }

    let current_time = std::time::Instant::now();
    let delta_time = current_time - self.last_time;
    if delta_time.as_secs_f32() > 5.0 {
      state.game_state = GameState::MainMenu;
    }
  }
}
