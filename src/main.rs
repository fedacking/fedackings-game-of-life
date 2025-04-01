mod gol;
use std::time::{Duration, Instant};

use macroquad::prelude::*;

use gol::{CellState, GameOfLife};

// UI Magic variables
const CELL_SIZE: f32 = 15.0;
const UPS: f32 = 2.0;

// Given a CellState and a series of coordinates in the screen
// x, y (with the center on the top left going right bottom)
// Draws a rectangle with black borders and filled with
// green for alive cells and gray for dead cells.
// The size of the rectangle is given by CELL_SIZE
fn draw_cell(state: CellState, x: f32, y: f32) {
    let color = match state {
        CellState::Alive => GREEN,
        _ => GRAY,
    };
    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
    draw_line(x, y, x + CELL_SIZE, y, 2.0, BLACK);
    draw_line(x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE, 2.0, BLACK);
    draw_line(x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE, 2.0, BLACK);
    draw_line(x, y, x, y + CELL_SIZE, 2.0, BLACK);
}

// Traverses the board to draw it. To see the details of how the board is drawn,
// see function draw_cell
fn draw_board<const WIDTH: usize, const HEIGHT: usize>(gol: GameOfLife<WIDTH, HEIGHT>) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            draw_cell(gol[(x, y)], x as f32 * CELL_SIZE, y as f32 * CELL_SIZE);
        }
    }
}

#[macroquad::main("Fedacking's Game of Life")]
async fn main() {
    //const D: CellState = CellState::Dead;
    const A: CellState = CellState::Alive;

    // This block of codes defines the initial size of our Game of Life
    // If you want to change the size it's necessary to also change the
    // Contents, as it expects an array of the correct size
    // Or change to a ::new and set the specific changed cells by hand
    /*let mut gol: GameOfLife<10, 10> = GameOfLife::from_board([
        [D, D, D, D, D, D, D, D, D, D],
        [D, A, A, A, A, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
        [D, D, D, D, D, D, D, D, D, D],
    ]);*/
    let mut gol: GameOfLife<50, 50> = GameOfLife::new();
    // Left Block
    gol[(0, 4)] = A;
    gol[(1, 4)] = A;
    gol[(0, 5)] = A;
    gol[(1, 5)] = A;

    // Right Block
    gol[(34, 3)] = A;
    gol[(35, 3)] = A;
    gol[(34, 4)] = A;
    gol[(35, 4)] = A;

    // Left Circle
    gol[(10, 4)] = A;
    gol[(10, 5)] = A;
    gol[(10, 6)] = A;
    gol[(11, 3)] = A;
    gol[(11, 7)] = A;
    gol[(12, 2)] = A;
    gol[(12, 8)] = A;
    gol[(13, 2)] = A;
    gol[(13, 8)] = A;

    let frame_time = 1000 / UPS as u64;
    let mut next_time = Instant::now() + Duration::from_millis(frame_time);
    loop {
        clear_background(BLACK);
        draw_board(gol);

        let now = Instant::now();
        if now > next_time {
            gol.update();
            next_time += Duration::from_millis(frame_time);
        }

        next_frame().await
    }
}
