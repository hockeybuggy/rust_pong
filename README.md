# Rust Pong

A toy project as an excuse to play with rust graphics.

It makes use of the piston game engine.

![A screen shot of the game in play][SCREENSHOT]

## Usage

Use cargo compile and run the game:

    git clone git@github.com:hockeybuggy/rust_pong.git
    cd rust_pong
    cargo run

## How to play.

The only current mode is two player. To move the left paddle use the `W` and
`S` keys. For the right paddle use the `Up` and `Down` keys. Attempt to get the
ball to hit the back of your opponents end.

## TODO

- Show score in graphics
- Improve ball paddle collisions (what happens if it hits the side of the paddle)
- Make paddles less clunky
- Increase ball speed
- Create AI to play against.
- Make the ball bounce in the right place.

[SCREENSHOT]: /pong-screenshot.gif
