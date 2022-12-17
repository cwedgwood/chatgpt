use std::{thread, time};
use rand::Rng;
use wasm_bindgen::prelude::*;

const ROWS: usize = 20;
const COLS: usize = 100;

// Structure to represent a cell in the grid
#[derive(Clone, Copy)]
struct Cell {
    is_alive: bool,
    num_live_neighbors: usize,
}

#[wasm_bindgen]
pub fn main() {
    // Create the grid of cells
    let mut grid = vec![vec![Cell { is_alive: false, num_live_neighbors: 0 }; COLS]; ROWS];

    // Initialize the grid
    initialize_grid(&mut grid);

    // Print the initial grid
    println!("Initial grid:");
    print_grid(&grid);

    // Update and print the grid for 10 iterations
    for _ in 0..10 {
        update_grid(&mut grid);
        println!("Updated grid:");
        print_grid(&grid);

        // Sleep for 500ms
        thread::sleep(time::Duration::from_millis(500));
    }
}

// Initializes the grid with random values
fn initialize_grid(grid: &mut Vec<Vec<Cell>>) {
    let mut rng = rand::thread_rng();
    for i in 0..ROWS {
        for j in 0..COLS {
            grid[i][j].is_alive = rng.gen();
            grid[i][j].num_live_neighbors = 0;
        }
    }
}

// Prints the grid to the console
fn print_grid(grid: &Vec<Vec<Cell>>) {
    for i in 0..ROWS {
        for j in 0..COLS {
            print!("{}", if grid[i][j].is_alive { 'X' } else { '.' });
        }
        println!();
    }
    println!();
}

// Updates the grid according to the rules of The Game of Life
fn update_grid(grid: &mut Vec<Vec<Cell>>) {
    // First, update the num_live_neighbors field for each cell
    for i in 0..ROWS {
        for j in 0..COLS {
            let mut num_live_neighbors = 0;

            // Check the 8 cells surrounding the current cell
            for k in -1..=1 {
                for l in -1..=1 {
                    // Skip the current cell
                    if k == 0 && l == 0 {
                        continue;
                    }

                    // Check if the neighboring cell is within the grid boundaries
                    if i as i64 + k >= 0 && i as i64 + k < ROWS as i64 && j as i64 + l >= 0 && j as i64 + l < COLS as i64 {
                        if grid[(i as i64 + k) as usize][(j as i64 + l) as usize].is_alive {
                            num_live_neighbors += 1;
                        }
                    }
                }
            }

            grid[i][j].num_live_neighbors = num_live_neighbors;
        }
    }

    // Next, update the state of each cell based on the number of live neighbors
    for i in 0..ROWS {
        for j in 0..COLS {
            let num_live_neighbors = grid[i][j].num_live_neighbors;

            // A live cell with fewer than 2 live neighbors dies (underpopulation)
            if grid[i][j].is_alive && num_live_neighbors < 2 {
                grid[i][j].is_alive = false;
            }
            // A live cell with 2 or 3 live neighbors lives on to the next generation
            else if grid[i][j].is_alive && (num_live_neighbors == 2 || num_live_neighbors == 3) {
                grid[i][j].is_alive = true;
            }
            // A live cell with more than 3 live neighbors dies (overpopulation)
            else if grid[i][j].is_alive && num_live_neighbors > 3 {
                grid[i][j].is_alive = false;
            }
            // A dead cell with exactly 3 live neighbors becomes a live cell (reproduction)
            else if !grid[i][j].is_alive && num_live_neighbors == 3 {
                grid[i][j].is_alive = true;
            }
        }
    }
}
