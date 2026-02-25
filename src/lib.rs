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

    pub fn render_tick<INPUT>(&mut self, delta_time: Duration, inputs: INPUT) -> &GameState
    where
        INPUT: IntoIterator<Item = Input>,
    {
        let mut total_time = Duration::ZERO;

        for input in inputs {
            self.update_tick(input.time_stamp, Some(input.action));
            total_time += input.time_stamp;
        }

        if total_time < delta_time {
            self.update_tick(delta_time - total_time, None);
        } else if total_time > delta_time {
            panic!("The input time_stamps exceeded the time since last frame");
        }

        &self.state
    }

    pub fn update_tick(&mut self, delta_time: Duration, input: Option<InputAction>) -> &GameState {
        match self.state.phase {
            GamePhase::GenerationPhase => {
                while self.state.space_for_bag() {
                    self.state.shuffle_new_bag(&mut self.rng);
                }

                // Generate a piece

                let piece: Tetromino;

                if !self.state.can_hold {
                    if let Some(hold_piece) = self.state.hold_piece {
                        piece = hold_piece;
                    } else {
                        piece = self.state.piece_queue.pop();
                    }

                    self.state.hold_piece = Some(piece);
                } else {
                    piece = self.state.piece_queue.pop();
                }

                self.state.generate_new_piece(piece);

                // Drop active piece one row and check for collisions etc...

                self.state.try_move(0, -1);

                // Enter fall state

                self.state.phase = GamePhase::FallingPhase;
            }
            GamePhase::FallingPhase => {
                // TODO: Handle input

                match input {
                    Some(InputAction::Left) => {
                        self.state.try_move(-1, 0);
                    }
                    Some(InputAction::Right) => {
                        self.state.try_move(1, 0);
                    }
                    Some(InputAction::RotateClockwise) => {
                        self.state.try_rotate(true);
                    }
                    Some(InputAction::RotateCounterclockwise) => {
                        self.state.try_rotate(false);
                    }
                    Some(InputAction::Hold) => {
                        if self.state.can_hold {
                            self.state.can_hold = false;

                            self.state.phase = GamePhase::GenerationPhase;
                        }
                    }
                    _ => (),
                }

                // TODO: When it locks down in lock phase, it should set self.state.can_hold = true;

                // TODO: Try to drop and enter lock phase if hit ground

                if self.state.simulate_piece(delta_time) {
                    self.state.phase = GamePhase::LockPhase;
                }
            }
            // TODO: finish all the GamePhases
            _ => (),
        }

        &self.state
    }
}

pub struct Input {
    pub action: InputAction,
    pub time_stamp: Duration,
}

impl Input {
    pub fn new(action: InputAction, time_stamp: Duration) -> Self {
        Self { action, time_stamp }
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
}
