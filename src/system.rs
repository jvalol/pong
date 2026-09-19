use crate::any;
use crate::input::Input;
use crate::pong_game::*;
use crate::state::*;
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
    state.player1.update_y_position(0.0);
    state.player2.update_y_position(0.0);
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

    if input.p1_up_pressed {
      let position = (
        state.player1.position().x,
        state.player1.position().y + util::PLAYER_SPEED * state.delta_time,
      );
      state.player1.update_position(position.into());
    }
    if input.p1_down_pressed {
      let position = (
        state.player1.position().x,
        state.player1.position().y - util::PLAYER_SPEED * state.delta_time,
      );
      state.player1.update_position(position.into());
    }
    if input.p2_up_pressed {
      let position = (
        state.player2.position().x,
        state.player2.position().y + util::PLAYER_SPEED * state.delta_time,
      );
      state.player2.update_position(position.into());
    }
    if input.p2_down_pressed {
      let position = (
        state.player2.position().x,
        state.player2.position().y - util::PLAYER_SPEED * state.delta_time,
      );
      state.player2.update_position(position.into());
    }

    // normalize players
    if state.player1.position().y > 1.0 - state.player1.size().y * 0.5 {
      let position = (
        state.player1.position().x,
        1.0 - state.player1.size().y * 0.5,
      );
      state.player1.update_position(position.into());
    } else if state.player1.position().y < state.player1.size().y * 0.5 - 1.0 {
      let position = (
        state.player1.position().x,
        state.player1.size().y * 0.5 - 1.0,
      );
      state.player1.update_position(position.into());
    }
    if state.player2.position().y > 1.0 - state.player2.size().y * 0.5 {
      let position = (
        state.player2.position().x,
        1.0 - state.player2.size().y * 0.5,
      );
      state.player2.update_position(position.into());
    } else if state.player2.position().y < state.player2.size().y * 0.5 - 1.0 {
      let position = (
        state.player2.position().x,
        state.player2.size().y * 0.5 - 1.0,
      );
      state.player2.update_position(position.into());
    }
  }
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
      state.ball.velocity = util::calc_ball_velocity(&state.ball, &state.player1);
    } else if state.player2.contains(&state.ball) {
      events.push(Event::BallBounce(state.ball.position()));
      // move the ball out to the paddle's left face so it doesn't bounce again next frame
      let x = state.player2.position().x - state.player2.size().x * 0.5 - state.ball.radius();
      state.ball.update_position((x, state.ball.position().y).into());
      state.ball.velocity = util::calc_ball_velocity(&state.ball, &state.player2);
    }

    state
      .ball
      .update_position(state.ball.position() + state.ball.velocity * state.delta_time);
    if state.ball.position().y > 1.0 {
      events.push(Event::BallBounce(state.ball.position()));
      state.ball.update_position((state.ball.position().x, 1.0).into());
      state.ball.velocity.y *= -1.0;
    } else if state.ball.position().y < -1.0 {
      events.push(Event::BallBounce(state.ball.position()));
      state.ball.update_position((state.ball.position().x, -1.0).into());
      state.ball.velocity.y *= -1.0;
    }

    if state.ball.position().x > 1.0 {
      state.player1.score += 1;
      if state.player1.score >= 5 {
        state.game_state = GameState::GameOver;
      } else {
        state.game_state = GameState::Serving;
        events.push(Event::Score(0));
      }
    } else if state.ball.position().x < -1.0 {
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
    let direction = state.ball.position().x.signum();
    state.ball.update_position((0.0, 0.0).into());
    state.ball.velocity = cgmath::Vector2::unit_x() * direction * -util::BALL_SPEED;
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
