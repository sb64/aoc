pub mod p1;
pub mod p2;

const PIECES: [([u8; 4], usize, usize); 5] = [
    ([0x3c, 0, 0, 0], 4, 1),
    ([0x10, 0x38, 0x10, 0], 3, 3),
    ([0x38, 0x8, 0x8, 0], 3, 3),
    ([0x20, 0x20, 0x20, 0x20], 1, 4),
    ([0x30, 0x30, 0, 0], 2, 2),
];

#[derive(Debug, Clone)]
struct Board<'a> {
    board: Vec<u8>,
    board_size: usize,
    num_rows_to_keep: usize,

    directions: &'a [u8],
    direction_index: usize,
    piece_index: usize,

    num_rows_trimmed: usize,
    max_height: usize,
}

impl<'a> Board<'a> {
    fn new(size: usize, num_rows_to_keep: usize, directions: &'a [u8]) -> Board<'a> {
        Self {
            board: vec![0; size],
            board_size: size,
            num_rows_to_keep,

            directions: directions,
            direction_index: 0,
            piece_index: 0,

            num_rows_trimmed: 0,
            max_height: 0,
        }
    }

    fn drop_pieces(&mut self, num_pieces: usize) {
        for _ in 0..num_pieces {
            self.drop_piece()
        }
    }

    fn drop_piece(&mut self) {
        let (mut cur_piece, width, height) = PIECES[self.piece_index];
        let mut x = 2;
        let mut y = self.max_height + 3;
        if y + 4 >= self.board_size {
            self.board.copy_within(
                (self.max_height - self.num_rows_to_keep)..self.max_height,
                0,
            );
            for row in &mut self.board[self.num_rows_to_keep..] {
                *row = 0;
            }
            self.num_rows_trimmed += self.max_height - self.num_rows_to_keep;
            self.max_height = self.num_rows_to_keep;
            y = self.max_height + 3;
        }

        loop {
            match self.directions[self.direction_index] {
                b'<' if x > 0
                    && cur_piece
                        .iter()
                        .enumerate()
                        .all(|(dy, row)| self.board[y + dy] & (row << 1) == 0) =>
                {
                    x -= 1;
                    for row in &mut cur_piece {
                        *row <<= 1;
                    }
                }
                b'>' if x + width < 7
                    && cur_piece
                        .iter()
                        .enumerate()
                        .all(|(dy, row)| self.board[y + dy] & (row >> 1) == 0) =>
                {
                    x += 1;
                    for row in &mut cur_piece {
                        *row >>= 1;
                    }
                }
                _ => {}
            }

            self.direction_index = (self.direction_index + 1) % self.directions.len();

            if cur_piece.iter().enumerate().all(|(dy, new_row)| {
                let Some(map_index) = (y + dy).checked_sub(1) else {
                    return false;
                };
                new_row & self.board[map_index] == 0
            }) {
                y -= 1;
            } else {
                for (dy, row) in cur_piece.iter().enumerate() {
                    self.board[y + dy] |= row;
                }
                self.max_height = self.max_height.max(y + height);
                break;
            }
        }

        self.piece_index = (self.piece_index + 1) % PIECES.len();
    }

    fn total_height(&self) -> usize {
        self.max_height + self.num_rows_trimmed
    }
}

const BOARD_SIZE: usize = 4096;
const NUM_ROWS_TO_KEEP: usize = 1024;
