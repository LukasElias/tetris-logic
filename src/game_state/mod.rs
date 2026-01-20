mod constants;

use crate::{Index, IndexMut, MaybeUninit, Rng, SliceRandom};
use constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum GamePhase {
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
            panic!(
                "We took it too far - Ninajirachi. You indexed too far into my Ring Buffer, you silly goose."
            )
        }

        let value: &MaybeUninit<T> = &self.buffer[(self.head + index) % self.capacity()];

        unsafe { value.assume_init_ref() }
    }
}

impl<T: Copy, const N: usize> IndexMut<usize> for RingBuffer<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "We took it too far - Ninajirachi. You indexed too far into my Ring Buffer, you silly goose."
            )
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

impl Tetromino {
    fn generation_coords(&self) -> (isize, isize) {
        match *self {
            Self::O => (4, 20), // 5th col, 21st row
            Self::I => (3, 18), // 4th col, 21st row
            _ => (3, 19),       // 4th col, 21st row
        }
    }
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
    pub phase: GamePhase,
    pub hold_piece: Option<Tetromino>,
    pub piece_queue: RingBuffer<Tetromino, PIECE_QUEUE_SIZE>,
    pub matrix: [[Option<Tetromino>; MATRIX_WIDTH]; MATRIX_HEIGHT],
    pub active_piece: ActivePiece,
    pub score: usize,
    pub lines: usize,
    pub level: usize,
}

impl GameState {
    // piece_queue

    pub fn space_for_bag(&self) -> bool {
        self.piece_queue.capacity() - self.piece_queue.len() >= 7
    }

    pub fn shuffle_new_bag<RNG: Rng>(&mut self, rng: &mut RNG) {
        if !self.space_for_bag() {
            panic!("Not enough space for a new bag");
        }

        let mut bag = ALL_TETROMINOS;

        bag.shuffle(rng);

        for piece in bag {
            self.piece_queue.push(piece);
        }
    }

    // matrix

    // active_piece

    pub fn generate_new_piece(&mut self, kind: Tetromino) {
        if self.phase != GamePhase::GenerationPhase {
            panic!("Tried to generate a tetromino outside of the generation phase. Somebody should fix this code");
        }

        let (x, y) = kind.generation_coords();

        self.active_piece.kind = kind;
        self.active_piece.rotation = Rotation::default();
        self.active_piece.x = x;
        self.active_piece.y = y;
    }

    // score

    // lines

    // level
}

impl Default for GameState {
    fn default() -> Self {
        // The active_piece is a dummy, since it'll get overwritten when the 7-bag gets shuffled and a piece
        // is generated in the GenerationPhase
        let active_piece = ActivePiece {
            kind: Tetromino::O,
            rotation: Rotation::default(),
            x: 0,
            y: 0,
        };

        Self {
            phase: GamePhase::default(),
            hold_piece: None,
            piece_queue: RingBuffer::new(),
            matrix: [[None; MATRIX_WIDTH]; MATRIX_HEIGHT],
            active_piece: active_piece,
            score: 0,
            lines: 0,
            level: 1,
        }
    }
}
