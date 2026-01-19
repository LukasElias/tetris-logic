#![no_std]

mod game_state;

use {
    game_state::{
        GamePhase,
    },
    core::{
        mem::MaybeUninit,
        ops::{Index, IndexMut},
    },
    rand::{Rng, seq::SliceRandom},
};

pub use game_state::GameState;

#[derive(Debug, Clone)]
pub struct Game<I: Input, R: Render, RNG: Rng> {
    state: GameState,
    input: I,
    render: R,
    rng: RNG,
}

impl<I: Input, R: Render, RNG: Rng> Game<I, R, RNG> {
    pub fn new(input: I, render: R, rng: RNG) -> Self {
        Self {
            state: GameState::default(),
            input,
            render,
            rng,
        }
    }

    pub fn tick(&mut self) {
        match self.state.phase() {
            GamePhase::GenerationPhase => {
                while self.state.space_for_bag() {
                    self.state.shuffle_new_bag(&mut self.rng);
                }

                // Generate a piece

                let piece = self.state.piece_queue_pop();
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    Left,
    Right,
    HardDrop,
    SoftDrop,
    RotateClockwise,
    RotateCounterclockwise,
    Hold,
    Pause,
}

pub trait Input {
    fn next_input(&mut self) -> Option<InputAction>;
}

pub trait Render {
    fn render(&mut self, state: &GameState);
}
