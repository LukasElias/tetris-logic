#![no_std]

use core::{mem::MaybeUninit, ops::{Index, IndexMut}};

const PIECE_QUEUE_SIZE: usize = 14;
const MATRIX_WIDTH: usize = 10;
const MATRIX_HEIGHT: usize = 20;

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

    pub fn push(&mut self, value: T) -> Result<T, ()> {
        if self.len == self.buffer.len() {
            return Err(());
        }

        self.buffer[(self.head + self.len) % self.capacity()].write(value);

        self.len += 1;

        Ok(value)
    }

    pub fn pop(&mut self) -> Result<T, ()> {
        if self.len == 0 {
            return Err(());
        }

        let value = self.buffer[self.head];
        self.head = (self.head + 1) % self.capacity();

        // There SHOULD be a value here, since you can only fill the buffer up using push.
        Ok(unsafe { value.assume_init() })
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

#[derive(Debug, Clone)]
pub struct Game<I: Input, R: Render> {
    state: GameState,
    input: I,
    render: R,
}

impl<I: Input, R: Render> Game<I, R> {
    pub fn tick(&mut self) {
        match self.state.phase {
            GamePhase::GenerationPhase => {

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
