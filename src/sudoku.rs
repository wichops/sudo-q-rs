use rand::seq::SliceRandom;
use rand::thread_rng;

const BOARD_SIZE: usize = 9;
type Board = [[i32; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug)]
struct Vec2D {
    row: usize,
    column: usize,
}

impl Vec2D {
    pub fn new(row: usize, column: usize) -> Vec2D {
        Vec2D { row, column }
    }
}
impl Eq for Vec2D {}
impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

pub struct Sudoku {
    pub board: Board,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut sudoku = Sudoku {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
        };

        sudoku.init_board();
        sudoku
    }

    fn init_board(&mut self) {
        let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut rng = thread_rng();
        numbers.shuffle(&mut rng);

        println!("seed: {:?}", numbers);
        let result = self.generate_solution(numbers, 0, 0);
        println!("Generated a puzzle: {}", result);
    }

    fn generate_solution(&mut self, seed: [i32; 9], row: usize, column: usize) -> bool {
        if row >= BOARD_SIZE || column >= BOARD_SIZE {
            return false;
        }

        for n in seed {
            self.board[row][column] = n;

            if self.is_valid_position(&Vec2D::new(row, column)) {
                if row == BOARD_SIZE - 1 && column == BOARD_SIZE - 1 {
                    return true;
                }

                if column == BOARD_SIZE - 1 {
                    if self.generate_solution(seed, row + 1, 0) {
                        return true;
                    }
                } else {
                    if self.generate_solution(seed, row, column + 1) {
                        return true;
                    }
                }
            }
        }

        self.board[row][column] = 0;

        false
    }

    fn is_valid_position(&self, position: &Vec2D) -> bool {
        !self.matches_row(position)
            && !self.matches_column(position)
            && !self.matches_quadrant(position)
    }

    fn matches_column(&self, position: &Vec2D) -> bool {
        for i in 0..BOARD_SIZE {
            let current_pos = &Vec2D::new(i, position.column);
            if position != current_pos {
                if self.board[position.row][position.column] == self.board[i][position.column] {
                    return true;
                }
            }
        }
        false
    }

    fn matches_row(&self, position: &Vec2D) -> bool {
        for i in 0..BOARD_SIZE {
            let current_pos = &Vec2D::new(position.row, i);
            if position != current_pos {
                if self.board[position.row][position.column] == self.board[position.row][i] {
                    return true;
                }
            }
        }
        false
    }

    fn matches_quadrant(&self, position: &Vec2D) -> bool {
        let quadrant = Vec2D::new(position.row / 3, position.column / 3);

        for i in 0..3 {
            for j in 0..3 {
                let row = 3 * quadrant.row + i;
                let column = 3 * quadrant.column + j;
                let current_pos = &Vec2D::new(row, column);

                if position != current_pos {
                    if self.board[position.row][position.column]
                        == self.board[current_pos.row][current_pos.column]
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}
