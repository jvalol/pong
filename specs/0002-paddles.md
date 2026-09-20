# 0002 Paddles

**Status:** implemented
**Date:** 2026-09-19

## Goal

Two players share a keyboard, and neither can drive a paddle off the screen.

## Behavior

W and S move the left paddle, Up and Down the right one. Keys are read as held
flags, so a paddle moves every frame the key is down. Both paddles move at 1.5
window heights per second, multiplied by the frame's delta time.

Up is negative y, since the origin is the window's top-left.

A paddle stops when its edge reaches the top or bottom of the window, so half its
height is the closest its center gets to either edge. Holding both directions at
once cancels out.

Keys are physical positions, so W and S are in the same place on QWERTY and QWERTZ.

## Acceptance criteria

- Pressing up moves a paddle toward the top of the window. — `system::tests::up_moves_the_paddle_up`
- Pressing down moves a paddle toward the bottom. — `system::tests::down_moves_the_paddle_down`
- Movement scales with delta time. — `system::tests::paddle_movement_scales_with_delta_time`
- A paddle stops at the top edge. — `system::tests::paddle_stops_at_the_top`
- A paddle stops at the bottom edge. — `system::tests::paddle_stops_at_the_bottom`
- Both directions at once leaves the paddle where it was. — `system::tests::opposite_keys_cancel`

### Verified by hand

- Both paddles respond at once, without one blocking the other. — run pong and
  have two people play.

## Out of scope

Paddle acceleration, a single player mode, and any AI opponent.
