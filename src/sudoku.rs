use rand::seq::SliceRandom;
use rand::thread_rng;

const BOARD_SIZE: usize = 9;
const QUADRANT_SIZE: usize = 3;
type Board = [[i32; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug, Clone)]
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

        sudoku.generate_puzzle();
        sudoku
    }

    fn generate_puzzle(&mut self) {
        let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut rng = thread_rng();
        numbers.shuffle(&mut rng);

        println!("seed: {:?}", numbers);
        let result = self.solve(numbers, &Vec2D::new(0, 0));

        println!("Generated a puzzle: {}", result);
    }

    fn solve(&mut self, seed: [i32; 9], position: &Vec2D) -> bool {
        let Vec2D { row, column } = *position;
        if row >= BOARD_SIZE || column >= BOARD_SIZE {
            return false;
        }

        let mut next_pos = position.clone();
        if column == BOARD_SIZE - 1 {
            next_pos.row += 1;
            next_pos.column = 0;
        } else {
            next_pos.column += 1;
        }

        for n in seed {
            self.board[row][column] = n;

            if self.is_valid_position(&Vec2D::new(row, column)) {
                println!("valid position, {}", n);
                if row == BOARD_SIZE - 1 && column == BOARD_SIZE - 1 {
                    return true;
                }

                if self.solve(seed, &next_pos) {
                    return true;
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
        let quadrant = Vec2D::new(
            position.row / QUADRANT_SIZE,
            position.column / QUADRANT_SIZE,
        );

        for i in 0..QUADRANT_SIZE {
            for j in 0..QUADRANT_SIZE {
                let row = QUADRANT_SIZE * quadrant.row + i;
                let column = QUADRANT_SIZE * quadrant.column + j;
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
