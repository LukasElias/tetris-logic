use crate::game_state::Tetromino;

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

// These are flipped, so that the 0, 0 is gonna be the bottom left of the bounding box.
pub const O_NORTH: [[bool; 2]; 2] = [
    [true, true],
    [true, true],
];
pub const I_NORTH: [[bool; 4]; 4] = [
    [false, false, false, false],
    [false, false, false, false],
    [true,  true,  true,  true ],
    [false, false, false, false],
];
pub const T_NORTH: [[bool; 3]; 3] = [
    [false, false, false],
    [true,  true,  true ],
    [false, true,  false],
];
pub const L_NORTH: [[bool; 3]; 3] = [
    [false, false, false],
    [true,  true,  true ],
    [false, false, true ],
];
pub const J_NORTH: [[bool; 3]; 3] = [
    [false, false, false],
    [true,  true,  true ],
    [true,  false, false],
];
pub const S_NORTH: [[bool; 3]; 3] = [
    [false, false, false],
    [true,  true,  false],
    [false, true,  true ],
];
pub const Z_NORTH: [[bool; 3]; 3] = [
    [false, false, false],
    [false, true,  false],
    [false, true,  true ],
];

