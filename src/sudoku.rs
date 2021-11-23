use rand::seq::SliceRandom;
use rand::thread_rng;

const EMPTY_LIMIT: i32 = 17;
const BOARD_SIZE: usize = 9;
const QUADRANT_SIZE: usize = 3;

pub type Board = [[i32; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug, Clone, Copy)]
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

#[derive(Clone, Default)]
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
        if result {
            self.empty_spaces();
        }
    }

    fn get_shuffled_positions(&self) -> Vec<Vec2D> {
        let mut positions = [Vec2D::new(0, 0); BOARD_SIZE * BOARD_SIZE];

        for (index, position) in positions.iter_mut().enumerate() {
            let row = index / 9;
            let column = index % 9;

            position.row = row;
            position.column = column;
        }

        let mut rng = thread_rng();
        positions.shuffle(&mut rng);

        positions.to_vec()
    }

    fn empty_spaces(&mut self) {
        let mut positions = self.get_shuffled_positions();

        while self.count_empty_spaces() <= EMPTY_LIMIT {
            let Vec2D { row, column } = positions.pop().unwrap();
            let board_value = self.board[row][column];

            self.board[row][column] = 0;
            let solutions = Sudoku::solve_multiple(&mut self.clone());

            if solutions > 1 {
                self.board[row][column] = board_value;
            }
        }
    }

    pub fn solve_multiple(&mut self) -> i32 {
        self.do_solve_multiple(Vec2D::new(0, 0), 0)
    }

    fn is_solved(&self) -> bool {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if !self.is_valid_position(&Vec2D::new(i, j)) {
                    return false;
                }
            }
        }
        true
    }

    fn count_empty_spaces(&self) -> i32 {
        let mut count = 0;
        for row in self.board {
            for col in row {
                if col == 0 {
                    count += 1;
                }
            }
        }
        count
    }

    fn has_empty_cells(&self) -> bool {
        for row in self.board {
            for col in row {
                if col == 0 {
                    return true;
                }
            }
        }
        false
    }

    fn do_solve_multiple(&mut self, position: Vec2D, solutions: i32) -> i32 {
        let Vec2D { row, column } = position;
        let mut current_solutions = solutions;

        if row >= BOARD_SIZE || column >= BOARD_SIZE {
            return solutions;
        }

        let mut next_pos = position;
        if column == BOARD_SIZE - 1 {
            next_pos.row += 1;
            next_pos.column = 0;
        } else {
            next_pos.column += 1;
        }

        if self.board[row][column] != 0 {
            return solutions + self.do_solve_multiple(next_pos, solutions);
        }

        for n in 1..=9 {
            self.board[row][column] = n;

            if !self.has_empty_cells() && self.is_solved() {
                current_solutions += 1;
            }

            if self.is_valid_position(&Vec2D::new(row, column)) {
                current_solutions += self.do_solve_multiple(next_pos, solutions);
            }
        }

        self.board[row][column] = 0;

        current_solutions
    }

    fn solve(&mut self, numbers: [i32; 9], position: &Vec2D) -> bool {
        let Vec2D { row, column } = *position;
        if row >= BOARD_SIZE || column >= BOARD_SIZE {
            return false;
        }

        let mut next_pos = *position;
        if column == BOARD_SIZE - 1 {
            next_pos.row += 1;
            next_pos.column = 0;
        } else {
            next_pos.column += 1;
        }

        for n in numbers {
            self.board[row][column] = n;

            if self.is_valid_position(&Vec2D::new(row, column)) {
                if row == BOARD_SIZE - 1 && column == BOARD_SIZE - 1 {
                    return true;
                }

                if self.solve(numbers, &next_pos) {
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
            if position != current_pos
                && self.board[position.row][position.column] == self.board[position.row][i]
            {
                return true;
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

                if position != current_pos
                    && self.board[position.row][position.column]
                        == self.board[current_pos.row][current_pos.column]
                {
                    return true;
                }
            }
        }
        false
    }
}
