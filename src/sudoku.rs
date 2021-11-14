const BOARD_SIZE: usize = 9;
type Board = [[u16; BOARD_SIZE]; BOARD_SIZE];

pub struct Sudoku {
    pub board: Board,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut sudoku = Sudoku {
            board: [[9; BOARD_SIZE]; BOARD_SIZE],
        };

        sudoku.init_board();
        sudoku
    }

    fn init_board(&mut self) {
        for (row_index, row) in self.board.iter_mut().enumerate() {
            for (col_index, col) in row.iter_mut().enumerate() {
                *col = ((row_index + 1) * (col_index + 1)) as u16;
            }
        }
    }
}
