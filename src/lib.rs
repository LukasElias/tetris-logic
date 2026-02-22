#![no_std]

mod game_state;

use {
    core::{
        mem::MaybeUninit,
        ops::{Index, IndexMut},
        time::Duration,
    },
    rand::{Rng, seq::SliceRandom},
};

pub use game_state::*;

#[derive(Debug, Clone)]
pub struct Game<RNG: Rng> {
    state: GameState,
    rng: RNG,
}

impl<RNG: Rng> Game<RNG> {
    pub fn new(rng: RNG) -> Self {
        Self {
            state: GameState::default(),
            rng,
        }
    }

    pub fn tick(&mut self, delta_time: Duration, input: Option<InputAction>) -> &GameState {
        match self.state.phase {
            GamePhase::GenerationPhase => {
                while self.state.space_for_bag() {
                    self.state.shuffle_new_bag(&mut self.rng);
                }

                // Generate a piece

                let piece = self.state.piece_queue.pop();

                self.state.generate_new_piece(piece);

                // Drop active piece one row and check for collisions etc...

                self.state.try_drop();

                // Enter fall state

                self.state.phase = GamePhase::FallingPhase;
            }
            GamePhase::FallingPhase => {
                // Handle input

                match input {
                    Some(_) => (),
                    None => (),
                }

                // Try to drop and enter lock phase if hit ground

                if self.state.simulate_piece(delta_time) {
                    self.state.phase = GamePhase::LockPhase;
                }
            }
            _ => (),
        }

        &self.state
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
