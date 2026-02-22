use crate::{Duration, game_state::Tetromino};

pub const PIECE_QUEUE_SIZE: usize = 14;
pub const MATRIX_WIDTH: usize = 10;
pub const MATRIX_HEIGHT: usize = 20;
pub const ALL_TETROMINOS: [Tetromino; 7] = [
    Tetromino::O,
    Tetromino::I,
    Tetromino::T,
    Tetromino::L,
    Tetromino::J,
    Tetromino::S,
    Tetromino::Z,
];
pub const LEVELS: usize = 15;
pub const FALL_TABLE: [Duration; LEVELS] = [
    Duration::from_millis(1000),
    Duration::from_millis(793),
    Duration::from_millis(618),
    Duration::from_millis(473),
    Duration::from_millis(355),
    Duration::from_millis(262),
    Duration::from_millis(190),
    Duration::from_millis(135),
    Duration::from_millis(94),
    Duration::from_millis(64),
    Duration::from_millis(43),
    Duration::from_millis(28),
    Duration::from_millis(18),
    Duration::from_millis(11),
    Duration::from_millis(7),
];

pub const TETROMINO_SHAPES: [[[(isize, isize); 4]; 7]; 4] = [
    [ // North
        [(0, 0), (0, 1), (1, 0), (1, 1)], // O
        [(0, 2), (1, 2), (2, 2), (3, 2)], // I
        [(0, 1), (1, 1), (2, 1), (1, 2)], // T
        [(0, 1), (1, 1), (2, 1), (2, 2)], // L
        [(0, 1), (1, 1), (2, 1), (0, 2)], // J
        [(0, 1), (1, 1), (1, 2), (2, 2)], // S
        [(0, 2), (1, 1), (1, 2), (2, 1)], // Z
    ],
    [ // East
        [(0, 0), (0, 1), (1, 0), (1, 1)], // O
        [(2, 0), (2, 1), (2, 2), (2, 3)], // I
        [(1, 0), (1, 1), (1, 2), (2, 1)], // T
        [(1, 0), (1, 1), (1, 2), (2, 0)], // L
        [(1, 0), (1, 1), (1, 2), (2, 2)], // J
        [(1, 2), (1, 1), (2, 1), (2, 0)], // S
        [(2, 2), (1, 1), (2, 1), (2, 1)], // Z
    ],
    [ // South
        [(0, 0), (0, 1), (1, 0), (1, 1)], // O
        [(0, 1), (1, 1), (2, 1), (3, 1)], // I
        [(0, 1), (1, 1), (2, 1), (1, 0)], // T
        [(0, 1), (1, 1), (2, 1), (0, 0)], // L
        [(0, 1), (1, 1), (2, 1), (2, 0)], // J
        [(2, 1), (1, 1), (1, 0), (0, 0)], // S
        [(2, 0), (1, 1), (1, 0), (0, 1)], // Z
    ],
    [ // West
        [(0, 0), (0, 1), (1, 0), (1, 1)], // O
        [(1, 0), (1, 1), (1, 2), (1, 3)], // I
        [(1, 0), (1, 1), (1, 2), (0, 1)], // T
        [(1, 0), (1, 1), (1, 2), (0, 2)], // L
        [(1, 0), (1, 1), (1, 2), (0, 0)], // J
        [(1, 0), (1, 1), (0, 1), (0, 2)], // S
        [(0, 0), (1, 1), (0, 1), (1, 2)], // Z
    ],
];
