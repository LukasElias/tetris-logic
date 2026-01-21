use tetris::{Game, Input, InputAction, Render, GameState};
use rand::rng;

fn main() {
    let input = InputDevice;
    let render = Renderer;
    let rng = rng();

    let mut game = Game::new(input, render, rng);

    game.tick();
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
