#![no_std]

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
pub struct RingBuffer<T, const S: usize> {
    buffer: [T; S],
    head: usize,
    len: usize,
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
