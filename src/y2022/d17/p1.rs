use super::{Board, BOARD_SIZE, NUM_ROWS_TO_KEEP};

pub fn solve(input: &str) -> eyre::Result<usize> {
    let mut board = Board::new(BOARD_SIZE, NUM_ROWS_TO_KEEP, input.trim().as_bytes());
    board.drop_pieces(2022);
    Ok(board.total_height())
}
