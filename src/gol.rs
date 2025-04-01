use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
}

impl CellState {
    fn pretty_print(self) -> char {
        match self {
            CellState::Alive => 'A',
            CellState::Dead => 'D',
        }
    }
}

fn update_cell(state: CellState, neighbours: usize) -> CellState {
    match (state, neighbours) {
        (CellState::Dead, 3) | (CellState::Alive, 2 | 3) => CellState::Alive,
        _ => CellState::Dead,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameOfLife<const WIDTH: usize, const HEIGHT: usize> {
    board: [[CellState; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<(usize, usize)> for GameOfLife<WIDTH, HEIGHT> {
    type Output = CellState;

    fn index(&self, index: (usize, usize)) -> &CellState {
        let (x, y) = index;
        &self.board[y][x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<(usize, usize)>
    for GameOfLife<WIDTH, HEIGHT>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut CellState {
        let (x, y) = index;
        &mut self.board[y][x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> GameOfLife<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        GameOfLife {
            board: [[CellState::Dead; WIDTH]; HEIGHT],
        }
    }

    // Allows you to start a game of life with a preset board
    pub fn from_board(board: [[CellState; WIDTH]; HEIGHT]) -> Self {
        GameOfLife { board }
    }

    // Prints the board to out in proper lines
    pub fn pretty_print(&self) {
        for board in self.board {
            println!("{:?}", board.map(CellState::pretty_print));
        }
    }

    fn y_permut(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut indexes: Vec<(usize, usize)> = vec![(x, y)];
        if y > 0 {
            indexes.push((x, y - 1));
        }
        if y < (HEIGHT - 1) {
            indexes.push((x, y + 1));
        }
        indexes
    }

    pub fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut indexes: Vec<(usize, usize)> = vec![];

        if x > 0 {
            indexes.extend(self.y_permut(x - 1, y));
        }
        if x < (WIDTH - 1) {
            indexes.extend(self.y_permut(x + 1, y));
        }
        indexes.extend(self.y_permut(x, y));
        indexes.retain(|index| *index != (x, y));

        indexes
            .iter()
            .filter(|cell| matches!(self[**cell], CellState::Alive))
            .count()
    }

    // This method updates the whole game of life to the next iteration
    pub fn update(&mut self) {
        let mut new_board = [[CellState::Dead; WIDTH]; HEIGHT];
        for (y, column) in self.board.iter().enumerate() {
            for (x, state) in column.iter().enumerate() {
                new_board[y][x] = update_cell(*state, self.count_neighbours(x, y))
            }
        }
        self.board = new_board;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> PartialEq for GameOfLife<WIDTH, HEIGHT> {
    fn eq(&self, other: &Self) -> bool {
        for (y, column) in self.board.iter().enumerate() {
            for (x, element) in column.iter().enumerate() {
                if *element != other[(x, y)] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const D: CellState = CellState::Dead;
    const A: CellState = CellState::Alive;

    #[test]
    // Verifies that the empty pattern should remove patterns
    fn test_empty() {
        let default_gol: GameOfLife<6, 6> = GameOfLife::new();
        let mut update_gol: GameOfLife<6, 6> = GameOfLife::new();
        update_gol.update();

        assert_eq!(default_gol, update_gol);
    }

    #[test]
    // Tests a pattern full of Alive cells. Only the corners should survive
    fn test_overcrowding() {
        let default_gol = GameOfLife::new();
        let middle_gol: GameOfLife<6, 6> = GameOfLife {
            board: [
                [A, D, D, D, D, A],
                [D, D, D, D, D, D],
                [D, D, D, D, D, D],
                [D, D, D, D, D, D],
                [D, D, D, D, D, D],
                [A, D, D, D, D, A],
            ],
        };
        let start_gol: GameOfLife<6, 6> = GameOfLife {
            board: [
                [A, A, A, A, A, A],
                [A, A, A, A, A, A],
                [A, A, A, A, A, A],
                [A, A, A, A, A, A],
                [A, A, A, A, A, A],
                [A, A, A, A, A, A],
            ],
        };
        let mut update_gol: GameOfLife<6, 6> = start_gol;
        update_gol.update();

        assert_eq!(middle_gol, update_gol);
        assert_ne!(start_gol, update_gol);
        update_gol.update();

        assert_eq!(default_gol, update_gol);
    }

    #[test]
    // Tests the cross repeating pattern. Each iteration returns to the previous state
    // DAD    DDD
    // DAD -> AAA
    // DDD    DDD
    fn test_repeat() {
        let a_gol: GameOfLife<3, 3> = GameOfLife {
            board: [[D, A, D], [D, A, D], [D, A, D]],
        };
        let b_gol: GameOfLife<3, 3> = GameOfLife::from_board([[D, D, D], [A, A, A], [D, D, D]]);
        let mut update_gol: GameOfLife<3, 3> = a_gol;

        assert_eq!(a_gol, update_gol);
        update_gol.update();
        assert_eq!(b_gol, update_gol);
        update_gol.update();
        assert_eq!(a_gol, update_gol);
        update_gol.update();
        assert_eq!(b_gol, update_gol);
    }

    #[test]
    // Tests the cube persistent pattern. It never changes
    // DDDD
    // DAAD
    // DAAD
    // DDDD
    fn test_persist() {
        let default_gol: GameOfLife<4, 4> = GameOfLife {
            board: [[D, D, D, D], [D, A, A, D], [D, A, A, D], [D, D, D, D]],
        };
        let mut update_gol: GameOfLife<4, 4> = default_gol;

        assert_eq!(default_gol, update_gol);
        update_gol.update();
        assert_eq!(default_gol, update_gol);
        update_gol.update();
        assert_eq!(default_gol, update_gol);
        update_gol.update();
        assert_eq!(default_gol, update_gol);
    }

    #[test]
    // Tests the transitions from cells
    fn test_transitions() {
        // Testing initial state D
        assert_eq!(D, update_cell(D, 1));
        assert_eq!(D, update_cell(D, 2));
        assert_eq!(A, update_cell(D, 3));
        assert_eq!(D, update_cell(D, 4));

        // Testing initial state A
        assert_eq!(D, update_cell(A, 1));
        assert_eq!(A, update_cell(A, 2));
        assert_eq!(A, update_cell(A, 3));
        assert_eq!(D, update_cell(A, 4));
    }
}
