use crate::y2022::d17::{Board, BOARD_SIZE, NUM_ROWS_TO_KEEP};

pub fn solve(input: &str) -> eyre::Result<usize> {
    let directions = input.trim().as_bytes();

    let pieces_period = if directions.len() % 5 == 0 {
        directions.len()
    } else {
        directions.len() * 5
    };

    let mut board = Board::new(BOARD_SIZE, NUM_ROWS_TO_KEEP, directions);
    board.drop_pieces(pieces_period);

    const MAX_REPEATS: usize = 1000;
    let mut delta_height = vec![board.total_height()];

    const SAMPLE_WINDOW: usize = 5;

    let mut period_height = 0;
    let mut period_length = 0;

    let mut synched_board = board.clone();

    for i in 1..MAX_REPEATS {
        let previous_height = board.total_height();
        board.drop_pieces(pieces_period);
        delta_height.push(board.total_height() - previous_height);

        if i > SAMPLE_WINDOW + 1 {
            let sample = &delta_height[(i - SAMPLE_WINDOW + 1)..(i + 1)];
            if sample == &delta_height[1..(1 + SAMPLE_WINDOW)] {
                let cycle_count = i - SAMPLE_WINDOW;
                period_length = cycle_count * pieces_period;

                for &dh in &delta_height[1..=cycle_count] {
                    period_height += dh;
                }

                break;
            }
        }
    }

    let mut pieces_remaining = 1000000000000 - pieces_period;
    let mut final_height = 0;

    let num_periods = pieces_remaining / period_length;
    final_height += period_height * num_periods;
    pieces_remaining %= period_length;

    synched_board.drop_pieces(pieces_remaining);
    final_height += synched_board.total_height();

    Ok(final_height)
}
