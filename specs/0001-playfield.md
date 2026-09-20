# 0001 Playfield

**Status:** implemented
**Date:** 2026-09-19

## Goal

The game fills whatever window it's given and plays the same at any size.

## Behavior

The window is the playfield. `State::field` holds its size in pixels, and
`State::layout` places everything relative to it:

- Paddles are 2.5% of the width by 20% of the height.
- The left paddle sits 10% in from the left, the right one 10% in from the right.
- The ball is square, 2.5% of the width on a side.
- The win text is centered in the window.

`layout` runs at startup and on every resize. Mid-game it scales the paddles, the
ball, and the ball's velocity by how much the window changed, so a rally survives
a resize instead of restarting or teleporting.

The score and menu text sit at fixed pixel offsets from the top-left, so they stay
put when the window grows.

## Acceptance criteria

- Paddles and ball are sized from the window. — `state::tests::layout_sizes_pieces_from_the_window`
- The ball is square whatever the window's shape. — `state::tests::ball_is_square_in_a_wide_window`
- Paddles sit 10% in from each side. — `state::tests::paddles_sit_inside_each_edge`
- The first layout centers the ball and paddles. — `state::tests::first_layout_centers_everything`
- A resize scales positions with the window. — `state::tests::resize_scales_positions`
- The win text follows the window's center. — `state::tests::layout_centers_win_text`

### Verified by hand

- Nothing stretches or jumps when the window is resized during a rally. — run pong
  and drag the window's corner mid-rally.

## Out of scope

Letterboxing, a fixed aspect ratio, and scaling text with the window.
