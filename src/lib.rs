#![no_std]

use core::{mem::MaybeUninit, ops::{Index, IndexMut}};
use rand::{Rng, seq::SliceRandom};

const PIECE_QUEUE_SIZE: usize = 14;
const MATRIX_WIDTH: usize = 10;
const MATRIX_HEIGHT: usize = 20;
const ALL_TETROMINOS: [Tetromino; 7] = [
    Tetromino::O,
    Tetromino::I,
    Tetromino::T,
    Tetromino::L,
    Tetromino::J,
    Tetromino::S,
    Tetromino::Z,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum GamePhase {
    #[default]
    GenerationPhase,
    FallingPhase,
    LockPhase,
    PatternPhase,
    // IterationPhase,
    // AnimationPhase,
    EliminatePhase,
    CompletionPhase,
}

#[derive(Debug, Clone)]
pub struct RingBuffer<T: Copy, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    head: usize,
    len: usize,
}

impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [MaybeUninit::uninit(); N],
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> T {
        if self.len == self.buffer.len() {
            panic!("The buffer is full, you can't push anymore.");
        }

        self.buffer[(self.head + self.len) % self.capacity()].write(value);

        self.len += 1;

        value
    }

    pub fn pop(&mut self) -> T {
        if self.len == 0 {
            panic!("There's nothing to pop");
        }

        let value = self.buffer[self.head];
        self.head = (self.head + 1) % self.capacity();

        // There SHOULD be a value here, since you can only fill the buffer up using push.
        unsafe { value.assume_init() }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

impl<T: Copy, const N: usize> Index<usize> for RingBuffer<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("We took it too far - Ninajirachi. You indexed too far into my Ring Buffer, you silly goose.")
        }

        let value: &MaybeUninit<T> = &self.buffer[(self.head + index) % self.capacity()];

        unsafe { value.assume_init_ref() }
    }
}

impl<T: Copy, const N: usize> IndexMut<usize> for RingBuffer<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("We took it too far - Ninajirachi. You indexed too far into my Ring Buffer, you silly goose.")
        }

        let value: &mut MaybeUninit<T> = &mut self.buffer[(self.head + index) % self.capacity()];

        unsafe { value.assume_init_mut() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tetromino {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Rotation {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePiece {
    kind: Tetromino,
    rotation: Rotation,
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
pub struct GameState {
    phase: GamePhase,
    hold: Option<Tetromino>,
    piece_queue: RingBuffer<Tetromino, PIECE_QUEUE_SIZE>,
    matrix: [[Option<Tetromino>; MATRIX_WIDTH]; MATRIX_HEIGHT],
    active_piece: ActivePiece,
    score: usize,
    lines: usize,
    level: usize,
}

impl GameState {
    fn shuffle<RNG: Rng>(&mut self, rng: &mut RNG) {
        if self.piece_queue.capacity() - self.piece_queue.len() < 7 {
            panic!("Not enough space for a new bag");
        }

        let mut bag = ALL_TETROMINOS;

        bag.shuffle(rng);

        for piece in bag {
            self.piece_queue.push(piece);
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        let active_piece = ActivePiece {
            kind: Tetromino::O,
            rotation: Rotation::default(),
            x: 0,
            y: 0,
        };

        Self {
            phase: GamePhase::default(),
            hold: None,
            piece_queue: RingBuffer::new(),
            matrix: [[None; MATRIX_WIDTH]; MATRIX_HEIGHT],
            active_piece: active_piece,
            score: 0,
            lines: 0,
            level: 1,
        }
    }
}

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
        match self.state.phase {
            GamePhase::GenerationPhase => {
                // Shuffle a new bag if the there's space for a new bag of 7

                while self.state.piece_queue.capacity() - self.state.piece_queue.len() >= 7 {
                    self.state.shuffle(&mut self.rng);
                }

                // Generate a piece

                self.state.active_piece.kind = self.state.piece_queue.pop();
            },
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
