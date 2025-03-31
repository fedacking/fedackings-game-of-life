mod gol;
use gol::{GameOfLife, CellState};

fn main() {
    let mut gol: GameOfLife<8, 8> = GameOfLife::new();
    gol[(3, 3)] = CellState::Alive;
    gol.pretty_print();
    println!("{:?}", gol.count_neighbours_all());
    gol.update();
    gol.pretty_print();
}
