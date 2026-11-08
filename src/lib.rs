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
    input_action_states: InputActionStates,
    rng: RNG,
    settings: Settings,
}

impl<RNG: Rng> Game<RNG> {
    pub fn new(rng: RNG) -> Self {
        Self {
            state: GameState::default(),
            rng,
            input_action_states: InputActionStates::default(),
            settings: Settings::default(),
        }
    }

    pub fn settings(mut self, settings: Settings) -> Self {
        self.settings = settings;

        self
    }

    pub fn render_tick<INPUT>(&mut self, delta_time: Duration, inputs: INPUT) -> &GameState
    where
        INPUT: IntoIterator<Item = Input>,
    {
        let mut total_time = Duration::ZERO;

        for input in inputs {
            match input.action {
                InputAction::Left => {
                    if let InputState::Pressed(_) = input.state && let InputState::Pressed(_) = self.input_action_states.left  {
                        panic!("got told the {:?} button was pressed, while it was already held down", input.action);
                    }

                    self.input_action_states.left = input.state;

                    self.update_tick(input.time_stamp, Some(input.action.into()));
                },
                InputAction::Right => {
                    if let InputState::Pressed(_) = input.state && let InputState::Pressed(_) = self.input_action_states.right  {
                        panic!("got told the {:?} button was pressed, while it was already held down", input.action);
                    }

                    self.input_action_states.right = input.state;

                    self.update_tick(input.time_stamp, Some(input.action.into()));
                },
                InputAction::SoftDrop => {
                    if let InputState::Pressed(_) = input.state && let InputState::Pressed(_) = self.input_action_states.soft_drop  {
                        panic!("got told the {:?} button was pressed, while it was already held down", input.action);
                    }

                    self.input_action_states.soft_drop = input.state;

                    self.update_tick(input.time_stamp, None);
                },
                InputAction::HardDrop
                | InputAction::RotateClockwise
                | InputAction::RotateCounterclockwise
                | InputAction::Hold => {
                    if let InputState::Released = input.state {
                        panic!("i don't care that the {:?} button got released", input.action);
                    }

                    self.update_tick(input.time_stamp, Some(input.action.into()));
                },
            }

            total_time += input.time_stamp;
        }

        if total_time < delta_time {
            self.update_tick(delta_time - total_time, None);
        } else if total_time > delta_time {
            panic!("The input time_stamps exceeded the time since last frame");
        }

        &self.state
    }

    fn update_tick(&mut self, delta_time: Duration, input: Option<OneshotAction>) -> &GameState {
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

                    self.state.hold_piece = Some(self.state.active_piece.kind);
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
                // update input timers

                self.input_action_states.update_timers(delta_time);

                // find out if the left or right was last pressed
                let latest = [
                    (InputAction::Left, &mut self.input_action_states.left),
                    (InputAction::Right, &mut self.input_action_states.right),
                ]
                .into_iter()
                .filter_map(|(action, state)| {
                    match state {
                        InputState::Pressed {
                            time_since_press,
                            time_simulated,
                        } => Some((action, time_since_press, time_simulated)),
                        _ => None,
                    }
                })
                .min_by_key(|(_, time_since_press, _)| **time_since_press);

                if let Some((InputAction::Left, time_since_press, time_simulated)) = latest {
                    while time_simulated < time_since_press {
                        // time_simulated = 0
                        //
                        // time_since_press 500 ms
                        //
                        // das 200
                        // arr 50
                        //
                        // find the sequence
                        //
                        // 200
                        // 50 * 6

                        // Check if we simulated the das move
                        // If yes we check if we have to simulate the das
                        //
                        // Ok it just came to my realization that this is wrong...
                        // I have to make the actions all part of the render_tick method.
                        // AAAAAAAAAAAAAAAAAAAHHHHHHHHHHHHHHHH
                        // Imma just commit it and work on something else.
                        if *time_simulated < self.settings.das {
                            
                        }
                    }
                } else if let Some((InputAction::Right, time_since_press, time_simulated)) = latest {
                }

                match input {
                    Some(OneshotAction::Left) => {
                        self.state.try_move(-1, 0);
                    }
                    Some(OneshotAction::Right) => {
                        self.state.try_move(1, 0);
                    }
                    Some(OneshotAction::RotateClockwise) => {
                        self.state.try_rotate(true);
                    }
                    Some(OneshotAction::RotateCounterclockwise) => {
                        self.state.try_rotate(false);
                    }
                    Some(OneshotAction::Hold) => {
                        if self.state.can_hold {
                            self.state.can_hold = false;

                            self.state.phase = GamePhase::GenerationPhase;
                        }
                    }
                    _ => (),
                }

                if self.state.simulate_piece(delta_time) {
                    self.state.phase = GamePhase::LockPhase;
                }
            }
            GamePhase::LockPhase => {
                // Set the timer if not set yet
                if self.state.active_piece.lockdown_timer.is_none() {
                    self.state.active_piece.reset_lockdown_timer();
                }

                // TODO: Implement the three drop rules
                match self.settings.lockdown {
                    LockdownRules::ExtendedPlacementLockDown => {}
                    LockdownRules::InfinitePlacementLockDown => {}
                    LockdownRules::ClassicLockDown => {}
                }

                // Increment timer
                self.state.active_piece.increment_lockdown_timer(delta_time);

                // Check if timer is done
                if self.state.active_piece.is_lockdown_timer_done() {
                    // lock down piece
                    if !self.state.try_lockdown() {
                        self.state.phase = GamePhase::GameOver;
                    } else {
                        self.state.phase = GamePhase::PatternPhase;
                    }
                }
            }
            GamePhase::PatternPhase => {
                for mino in self.state.active_piece.shape() {
                    if self.state.hit_list[mino.1 as usize].is_some() {
                        continue;
                    }

                    if self
                        .state
                        .line_completed((self.state.active_piece.y + mino.1) as usize)
                    {
                        self.state.hit_list[3 - mino.1 as usize] =
                            Some((self.state.active_piece.y + mino.1) as usize);
                    }
                }

                self.state.phase = GamePhase::EliminatePhase;
            }
            GamePhase::EliminatePhase => {
                // TODO: Optimize this thing

                for line in &mut self.state.hit_list {
                    if line.is_none() {
                        continue;
                    }

                    for y in line.unwrap()..self.state.matrix.len() {
                        let new_line = self.state.matrix.get(y + 1).unwrap_or(&[None; 10]);

                        self.state.matrix[y] = *new_line;
                    }

                    *line = None;
                }

                self.state.phase = GamePhase::GenerationPhase;
            }
            // TODO: finish all the GamePhases
            _ => (),
        }

        &self.state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Input {
    action: InputAction,
    state: InputState,
    time_stamp: Duration,
}

impl Input {
    pub fn release(action: InputAction, time_stamp: Duration) -> Self {
        Self {
            action,
            state: InputState::Released,
            time_stamp,
        }
    }

    pub fn press(action: InputAction, time_stamp: Duration) -> Self {
        Self {
            action,
            state: InputState::Pressed {
                time_since_press: Duration::ZERO,
                time_simulated: Duration::ZERO,
            },
            time_stamp,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum InputState {
    #[default]
    Released,
    Pressed {
        time_since_press: Duration,
        time_simulated: Duration,
    }
}

impl InputState {
    fn update_timer(&mut self, delta_time: Duration) {
        if let Self::Pressed { time_since_press, time_simulated } = self {
            *time_since_press += delta_time;
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
}

// Every action but SoftDrop, since it's more of a toggle.
// Left and Right are still oneshot actions since they happen once, but can also repeat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OneshotAction {
    Left,
    Right,
    HardDrop,
    RotateClockwise,
    RotateCounterclockwise,
    Hold,
}

impl From<InputAction> for OneshotAction {
    fn from(value: InputAction) -> Self {
        match value {
            InputAction::Left => Self::Left,
            InputAction::Right => Self::Right,
            InputAction::HardDrop => Self::HardDrop,
            InputAction::SoftDrop => panic!("soft drop isn't a oneshot action"),
            InputAction::RotateClockwise => Self::RotateClockwise,
            InputAction::RotateCounterclockwise => Self::RotateCounterclockwise,
            InputAction::Hold => Self::Hold,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct InputActionStates {
    soft_drop: InputState,
    left: InputState,
    right: InputState,
}

impl InputActionStates {
    fn update_timers(&mut self, delta_time: Duration) {
        self.soft_drop.update_timer(delta_time);
        self.left.update_timer(delta_time);
        self.right.update_timer(delta_time);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Settings {
    lockdown: LockdownRules,
    arr: Duration, // Time until next automatic repeat
    das: Duration, // Time until arr kicks in
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LockdownRules {
    #[default]
    ExtendedPlacementLockDown,
    InfinitePlacementLockDown,
    ClassicLockDown,
}
