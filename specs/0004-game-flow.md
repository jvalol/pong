# 0004 Game flow

**Status:** implemented
**Date:** 2026-09-20

## Goal

Getting in and out of a game is obvious, and nothing is left in a strange state.

## Behavior

**No menu.** It opens on the serve. There used to be a title with Play and Quit
in front of it, which made sense when a game was the only thing you could have
launched and makes none now: the arcade is the menu, and a splash screen in
front of a game you walked up to and started is a second front door.

**Serving.** After a point the ball waits in the middle for two seconds, then play
starts. The scores are redrawn here.

**Playing.** The rally, per spec 0003.

**Paused.** Losing window focus during play pauses the game and shows one line
on a panel. Enter carries on, and Escape quits the way it does during play, so a
pause is not the one state the key stops working in. Losing focus anywhere else,
such as on the game over screen, changes nothing.

**Game over.** The winner is named on a panel until Enter serves a fresh match,
scores back to nothing. It used to count five seconds down to the menu; with no
menu to return to, it waits to be asked, the way cascada and carom already do.
Escape quits from here.

**Escape** quits from anywhere. It is acted on once per press, so holding it
cannot quit twice.

## Acceptance criteria

- Escape during play quits. — `system::tests::escape_quits_a_game_in_play`
- Losing focus while playing pauses. — `pong_game::tests::losing_focus_while_playing_pauses`
- Losing focus while already over changes nothing. — `pong_game::tests::losing_focus_while_already_over_does_nothing`
- Resuming carries on where it stopped. — `system::tests::resuming_carries_on_where_it_stopped`
- A finished match waits to be asked. — `system::tests::a_finished_match_waits_to_be_asked`
- Escape leaves a paused game. — `system::tests::escape_leaves_a_paused_game`
- And quits rather than backing out. — `pong_game::tests::escaping_out_of_a_pause_quits`
- Escape is ignored on key repeat. — `input::tests::escape_ignores_key_repeat`
- Escape is ignored on release. — `input::tests::escape_ignores_release`

### Verified by hand

- The five second game over wait and the two second serve feel right. — run pong
  and play a full game.

## Out of scope

A pause key, a settings screen, and remembering scores between runs.
