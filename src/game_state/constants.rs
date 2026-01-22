use crate::{Duration, game_state::Tetromino};
use paste::paste;

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

macro_rules! create_luts {
    ($size:literal, $tetromino:ident, $lut:expr) => {
        paste! {
            pub const [<$tetromino _OCCUPANCY_LUTS>]: [[[bool; $size]; $size]; 4] = gen_rotations($lut);

            pub const [<$tetromino _LOWEST_MINO>]: [[Option<usize>; $size]; 4] = gen_lowest([<$tetromino _OCCUPANCY_LUTS>]);
        }
    };
}

const fn gen_rotations<const N: usize>(north: [[bool; N]; N]) -> [[[bool; N]; N]; 4] {
    let east = rotate_lut(north);
    let south = rotate_lut(east);
    let west = rotate_lut(south);

    [north, east, south, west]
}

const fn gen_lowest<const N: usize>(shapes: [[[bool; N]; N]; 4]) -> [[Option<usize>; N]; 4] {
    [
        lowest_mino(shapes[0]),
        lowest_mino(shapes[1]),
        lowest_mino(shapes[2]),
        lowest_mino(shapes[3]),
    ]
}

const fn rotate_lut<const N: usize>(lut: [[bool; N]; N]) -> [[bool; N]; N] {
    let mut out = [[false; N]; N];

    let mut y = 0;
    while y < N {
        let mut x = 0;
        while x < N {
            out[x][N - 1 - y] = lut[y][x];
            x += 1;
        }
        y += 1;
    }

    out
}

const fn lowest_mino<const N: usize>(shape: [[bool; N]; N]) -> [Option<usize>; N] {
    let mut out = [None; N];
    let mut x = 0;
    while x < N {
        let mut y = 0;
        while y < N {
            if shape[y][x] {
                out[x] = Some(y);
                break;
            }
            y += 1;
        }
        x += 1;
    }
    out
}

// O

create_luts!(2, O, [[true, true], [true, true],]);

// I

create_luts!(
    4,
    I,
    [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
    ]
);

// T

create_luts!(
    3,
    T,
    [
        [false, false, false],
        [true, true, true],
        [false, true, false],
    ]
);

// L

create_luts!(
    3,
    L,
    [
        [false, false, false],
        [true, true, true],
        [false, false, true],
    ]
);

// J

create_luts!(
    3,
    J,
    [
        [false, false, false],
        [true, true, true],
        [true, false, false],
    ]
);

// S

create_luts!(
    3,
    S,
    [
        [false, false, false],
        [true, true, false],
        [false, true, true],
    ]
);

// Z

create_luts!(
    3,
    Z,
    [
        [false, false, false],
        [false, true, true],
        [true, true, false],
    ]
);
