#![no_std]

const PIECE_QUEUE_SIZE: usize = 14;
const MATRIX_WIDTH: usize = 10;
const MATRIX_HEIGHT: usize = 20;

pub struct RingBuffer<T, const C: usize> {
    buffer: [T; C],
    head: usize,
    len: usize,
}

pub enum Tetromino {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

pub enum Rotation {
    North,
    East,
    South,
    West,
}

pub struct ActivePiece {
    kind: Tetromino,
    rotation: Rotation,
    x: isize,
    y: isize,
}

pub struct GameState {
    hold: Option<Tetromino>,
    piece_queue: RingBuffer<Tetromino, PIECE_QUEUE_SIZE>,
    matrix: [[Tetromino; MATRIX_WIDTH]; MATRIX_HEIGHT],
    active_piece: ActivePiece,
    score: usize,
    lines: usize,
    level: usize,
}

pub struct Game<I: Input, R: Render> {
    state: GameState,
    input: I,
    render: R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputAction {
    #[default]
    None,
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
    fn next_input(&mut self) -> InputAction;
}

pub trait Render {
    fn render(&mut self, state: &GameState);
}
