mod constants;

use crate::{Duration, Index, IndexMut, MaybeUninit, Rng, SliceRandom};
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

impl Rotation {
    pub fn clockwise(&self) -> Self {
        match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        match *self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn rotate(&self, clockwise: bool) -> Self {
        if clockwise {
            self.clockwise()
        } else {
            self.counter_clockwise()
        }
    }

    fn kick_table_index(&self, clockwise: bool) -> usize {
        (*self as usize * 2 + 8 - !clockwise as usize) % 8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePiece {
    pub kind: Tetromino,
    pub rotation: Rotation,
    pub x: isize,
    pub y: isize,
    pub time_existed: Duration,
    pub time_simulated: Duration,
    pub lockdown_timer: Option<Duration>,
}

impl ActivePiece {
    pub fn shape(&self) -> [(isize, isize); 4] {
        TETROMINO_SHAPES[self.rotation as usize][self.kind as usize]
    }

    pub fn shape_rotation(&self, rotation: Rotation) -> [(isize, isize); 4] {
        TETROMINO_SHAPES[rotation as usize][self.kind as usize]
    }

    pub fn reset_lockdown_timer(&mut self) {
        self.lockdown_timer = Some(Duration::from_millis(0));
    }

    pub fn increment_lockdown_timer(&mut self, time: Duration) {
        *self.lockdown_timer.as_mut().unwrap() += time;
    }

    pub fn is_lockdown_timer_done(&self) -> bool {
        if let Some(timer) = self.lockdown_timer {
            timer >= LOCKDOWN_TIME
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub phase: GamePhase,
    pub hold_piece: Option<Tetromino>,
    pub can_hold: bool,
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

    pub fn out_of_bounds(x: isize, y: isize) -> bool {
        x < 0 || x >= MATRIX_WIDTH as isize || y < 0 || y >= MATRIX_HEIGHT as isize
    }

    pub fn is_empty(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= MATRIX_WIDTH as isize || y < 0 {
            return false;
        } else if y >= MATRIX_HEIGHT as isize {
            return true;
        }

        self.matrix[y as usize][x as usize].is_none()
    }

    pub fn render_matrix(&self) -> [[Option<Tetromino>; MATRIX_WIDTH]; MATRIX_HEIGHT] {
        let mut matrix = self.matrix.clone();

        for mino in self.active_piece.shape() {
            let (x, y) = (self.active_piece.x + mino.0, self.active_piece.y + mino.1);

            if GameState::out_of_bounds(x, y) {
                continue;
            }

            matrix[y as usize][x as usize] = Some(self.active_piece.kind);
        }

        matrix
    }

    // active_piece

    pub fn generate_new_piece(&mut self, kind: Tetromino) {
        if self.phase != GamePhase::GenerationPhase {
            panic!(
                "Tried to generate a tetromino outside of the generation phase. Somebody should fix this code"
            );
        }

        let (x, y) = kind.generation_coords();

        self.active_piece.kind = kind;
        self.active_piece.rotation = Rotation::default();
        self.active_piece.x = x;
        self.active_piece.y = y;
        self.active_piece.time_existed = Duration::ZERO;
        self.active_piece.time_simulated = Duration::ZERO;
    }

    pub fn can_move(&self, x: isize, y: isize) -> bool {
        self.can_move_rotation(x, y, self.active_piece.rotation)
    }

    pub fn can_move_rotation(&self, x: isize, y: isize, rotation: Rotation) -> bool {
        for mino in self.active_piece.shape_rotation(rotation) {
            if !self.is_empty(x + mino.0, y + mino.1) {
                return false;
            }
        }

        true
    }

    pub fn try_move(&mut self, x: isize, y: isize) -> bool {
        if self.can_move(self.active_piece.x + x, self.active_piece.y + y) {
            self.active_piece.x += x;
            self.active_piece.y += y;
            return true;
        }

        return false;
    }

    pub fn simulate_piece(&mut self, delta_time: Duration) -> bool {
        let mut on_surface = false;
        self.active_piece.time_existed += delta_time;

        while self.active_piece.time_existed > self.active_piece.time_simulated {
            if !self.try_move(0, -1) {
                on_surface = true;
            }

            self.active_piece.time_simulated += self.fall_speed();
        }

        on_surface
    }

    pub fn can_rotate(&self, clockwise: bool) -> Option<(isize, isize)> {
        // This method returns if you can rotate, and what kick you should do in SRS

        match self.active_piece.kind {
            Tetromino::O => Some((0, 0)),
            Tetromino::I => {
                let kick_table =
                    I_WALL_KICK_TABLE[self.active_piece.rotation.kick_table_index(clockwise)];

                for kick in kick_table {
                    if self.can_move_rotation(
                        self.active_piece.x + kick.0,
                        self.active_piece.y + kick.1,
                        self.active_piece.rotation.rotate(clockwise),
                    ) {
                        return Some(kick);
                    }
                }

                None
            }
            _ => {
                let kick_table =
                    JLSTZ_WALL_KICK_TABLE[self.active_piece.rotation.kick_table_index(clockwise)];

                for kick in kick_table {
                    if self.can_move_rotation(
                        self.active_piece.x + kick.0,
                        self.active_piece.y + kick.1,
                        self.active_piece.rotation.rotate(clockwise),
                    ) {
                        return Some(kick);
                    }
                }

                None
            }
        }
    }

    pub fn try_rotate(&mut self, clockwise: bool) -> bool {
        if let Some(kick) = self.can_rotate(clockwise) {
            self.active_piece.rotation = self.active_piece.rotation.rotate(clockwise);
            self.active_piece.x += kick.0;
            self.active_piece.y += kick.1;

            return true;
        }

        false
    }

    // lockdown

    pub fn lockdown(&mut self) {
        for mino in self.active_piece.shape() {
            let (x, y) = (
                (self.active_piece.x + mino.0) as usize,
                (self.active_piece.y + mino.1) as usize,
            );

            self.matrix[y][x] = Some(self.active_piece.kind);
        }

        self.can_hold = true;
    }

    // score

    // lines

    // level

    pub fn fall_speed(&self) -> Duration {
        // (0.8 - ((self.level as f32 - 1.0) * 0.007)).powf(self.level as f32 - 1.0)
        // amount of time for a single drop

        if self.level > LEVELS {
            return FALL_TABLE[LEVELS - 1];
        }

        FALL_TABLE[self.level - 1]
    }
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
            time_existed: Duration::ZERO,
            time_simulated: Duration::ZERO,
            lockdown_timer: None,
        };

        Self {
            phase: GamePhase::default(),
            hold_piece: None,
            can_hold: true,
            piece_queue: RingBuffer::new(),
            matrix: [[None; MATRIX_WIDTH]; MATRIX_HEIGHT],
            active_piece: active_piece,
            score: 0,
            lines: 0,
            level: 1,
        }
    }
}
