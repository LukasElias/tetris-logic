use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rand::rng;
use tetris::{Game, GameState, Input, InputAction, Render};

fn main() {
    let input = InputDevice;
    let render = Renderer;
    let rng = rng();

    let mut game = Game::new(input, render, rng);

    let mut last_tick = Instant::now();
    loop {
        game.tick(last_tick.elapsed());

        sleep(Duration::from_millis(10));

        last_tick = Instant::now();
    }
}

struct InputDevice;

impl Input for InputDevice {
    fn next_input(&mut self) -> Option<InputAction> {
        None
    }
}

struct Renderer;

impl Render for Renderer {
    fn render(&mut self, state: &GameState) {
        println!("{:?}", state);
    }
}
