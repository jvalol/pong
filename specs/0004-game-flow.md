# 0004 Game flow

**Status:** implemented
**Date:** 2026-09-20

## Goal

Getting in and out of a game is obvious, and nothing is left in a strange state.

## Behavior

**Menu.** PONG with Play and Quit. Up and Down move between them, Enter chooses,
and Escape quits.

**Serving.** After a point the ball waits in the middle for two seconds, then play
starts. The scores are redrawn here.

**Playing.** The rally, per spec 0003.

**Paused.** Losing window focus during play pauses the game and shows Paused with
Resume. Enter resumes and puts the menu's own wording back. Losing focus anywhere
else, such as on the menu or the game over screen, changes nothing.

**Game over.** The winner is named for five seconds, then the game returns to the
menu. Escape quits from here.

**Escape** during serving or play returns to the menu and resets the scores and
paddles, so the next game starts clean. Escape is acted on once per press, and
holding it doesn't carry into the menu and quit the game.

## Acceptance criteria

- Escape during play returns to the menu. — `system::tests::escape_returns_to_the_menu`
- Escape from the menu quits. — `system::tests::escape_quits_from_the_menu`
- Returning to the menu resets the score. — `system::tests::returning_to_the_menu_resets_the_score`
- Losing focus while playing pauses. — `pong_game::tests::losing_focus_while_playing_pauses`
- Losing focus on the menu changes nothing. — `pong_game::tests::losing_focus_on_the_menu_does_nothing`
- Resuming restores the menu wording. — `system::tests::resuming_restores_the_menu_text`
- Escape is ignored on key repeat. — `input::tests::escape_ignores_key_repeat`
- Escape is ignored on release. — `input::tests::escape_ignores_release`

### Verified by hand

- The five second game over wait and the two second serve feel right. — run pong
  and play a full game.

## Out of scope

A pause key, a settings screen, and remembering scores between runs.
