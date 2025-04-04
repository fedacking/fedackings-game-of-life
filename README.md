# Fedacking's Game of Life

An implementation of rust implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) with a GUI to observe it run.

## Game of Life

### Explanation

From [Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life):

>    The universe of the Game of Life is an infinite two-dimensional orthogonal grid of square cells, each of which is in one of two possible states, alive or dead, or "populated" or "unpopulated". Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions occur:

>        Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.

>        Any live cell with two or three live neighbours lives on to the next generation.

>        Any live cell with more than three live neighbours dies, as if by overpopulation.

>        Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

>    The initial pattern constitutes the seed of the system. The first generation is created by applying the above rules simultaneously to every cell in the seed—births and deaths occur simultaneously, and the discrete moment at which this happens is sometimes called a tick (in other words, each generation is a pure function of the preceding one). The rules continue to be applied repeatedly to create further generations.

### Constraints

Because we have limited memory, our game of life is limited to the a square of size defined at compile time in our main.rs source file. 

The Game of life comes with a graphical user interface that updates at the rate of 2 times per second. This and the size of the cells on screen can be changed through consts in main.rs (respectively UPS and CELL_SIZE).

## Dependencies

* [Macroquad](https://github.com/not-fl3/macroquad)

## Running

You can run the game by using:

```
make run
```

Tests can be executed with:

```
make test
```
