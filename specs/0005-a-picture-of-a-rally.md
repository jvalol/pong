# 0005 A picture of a rally

**Status:** implemented
**Date:** 2026-11-03

## Goal

The frame pong puts on its readme is held against a picture of what that
frame should be.

## Why it belongs here

Specs 0001 to 0004 are all arithmetic: where a paddle is, which way the ball
leaves, who scores, what the state machine does next. None of them can see
the table.

**And the engine's first harness could not either.** `drawn_looks_right`
draws the 3D and leaves the overlay empty, which is right for a game with a
world behind the help text. pong has no world. Every quad it owns goes into
`Geometry`, so that call photographs a black frame: a picture that passes
every check there is and says nothing. The engine's spec 0078 is the call
that takes the overlay, and this is the first game to use it.

## Behavior

**`PongGame::new` takes `staged` rather than reading the arguments.** A
picture test asks for the photographed state without `--screenshot` in its
own command line. `main` passes what the arguments say.

**One `update`, at no time at all.** `--screenshot` serves the game to
itself on the first update, because a photograph of the main menu says
nothing about pong. A `dt` of nought means the ball does not travel between
that and the shutter.

**It is the engine's size**, 320 by 240, said once in `picture::SIZE`, and
the engine's `SPARSE` share rather than its `SHARE`, which the overlay call
picks on its own.

## Acceptance criteria

- A rally in play matches its
  picture. — `pong_game::tests::a_rally_in_play_looks_right`

### Verified by hand

- **The picture itself.** Two paddles, the ball at the middle, nought to
  nought above. The same frame the committed `media/screenshot.png` shows.
- **A control, and what it takes.** The paddles grown from a fifth of the
  window's height to just under a quarter failed on 0.2% of the pixels,
  four times the share the engine allows a flat game. Grown by a twentieth
  instead, they passed: that is two pixels on a paddle eight wide, about
  38 pixels in all, and it sits on the line.

  So this holds pong to a paddle's rough size rather than to its exact one.
  A paddle missing, a ball that never serves, a score in the wrong corner,
  a layout flipped: all of those move far more. The engine's spec 0078 has
  why the share for a flat game is a quarter of what a 3D one gets, and
  what was measured to arrive at it.

## Out of scope

**The menu, the pause screen and the game over screen.** Each is a state
spec 0004 already tests, and a picture of each is four references that
change together every time the font does.
