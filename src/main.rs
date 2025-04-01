mod gol;

use gol::{CellState, GameOfLife};

fn main() {
    const D: CellState = CellState::Dead;
    const A: CellState = CellState::Alive;

    let mut gol: GameOfLife<5, 5> = GameOfLife::from_board([
        [D, D, D, D, D],
        [D, D, D, D, D],
        [D, A, A, A, D],
        [D, D, D, D, D],
        [D, D, D, D, D],
    ]);
    let _default_gol: GameOfLife<5, 5> = GameOfLife::new();
    gol.pretty_print();
    println!("");
    gol.update();
    gol.pretty_print();
    println!("");
    gol.update();
    gol.pretty_print();
}
