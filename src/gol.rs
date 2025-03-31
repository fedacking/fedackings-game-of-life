use std::{cell::Cell, ops::{Index, IndexMut}};

#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Alive, Dead
}

impl CellState {
    fn pretty_print(self) -> char {
        match self {
            CellState::Alive => 'A',
            CellState::Dead => 'D'
        }
    }
}

fn update_cell(state: CellState, neighbours: usize) -> CellState {
    match state {
        CellState::Alive => {
            match neighbours {
                2..3 => CellState::Alive,
                _ => CellState::Dead
            }
        },
        CellState::Dead => {
            match neighbours {
                3 => CellState::Alive,
                _ => CellState::Dead
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameOfLife<const WIDTH: usize, const HEIGHT: usize> { 
    board: [[CellState; WIDTH]; HEIGHT]
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<(usize, usize)> for GameOfLife<WIDTH, HEIGHT> {
    type Output = CellState;

    fn index(&self, index: (usize, usize)) -> &CellState {
        let (x, y) = index;
        &self.board[y][x]
    }
}   

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<(usize, usize)> for GameOfLife<WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut CellState {
        let (x, y) = index;
        &mut self.board[y][x]
    }
}   


impl<const WIDTH: usize, const HEIGHT: usize> GameOfLife<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        GameOfLife { board: [[CellState::Dead; WIDTH]; HEIGHT] }
    }

    pub fn pretty_print(&self) {
        for board in self.board {
            println!("{:?}", board.map(CellState::pretty_print));
        }
    }

    fn y_permut(&self,  x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut indexes: Vec<(usize, usize)> = vec![(x, y)];
        if y > 0 { indexes.push((x, y - 1));}
        if y < (HEIGHT - 1) { indexes.push((x, y + 1));}
        indexes
    }

    pub fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut indexes: Vec<(usize, usize)> = vec![];

        if  x > 0 { indexes.extend(self.y_permut(x - 1, y)); }
        if  x < (WIDTH - 1) { indexes.extend(self.y_permut(x + 1, y)); }
        indexes.extend(self.y_permut(x, y));
        indexes.retain(|index| *index != (x, y));

        indexes.iter().filter(|cell| matches!(self[**cell], CellState::Alive)).count()
    }

    pub fn count_neighbours_all(&self) -> [[usize; WIDTH]; HEIGHT]{
        let mut result = [[0; WIDTH]; HEIGHT];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[y][x] = self.count_neighbours(x, y);
            }
        }
        result
    }

    pub fn update(&mut self) {
        let mut new_board = [[CellState::Dead; WIDTH]; HEIGHT];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                new_board[y][x] = update_cell(self[(x, y)], self.count_neighbours(x, y))
            }
        }
        self.board = new_board;
    }
}
