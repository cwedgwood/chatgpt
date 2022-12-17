use std::{thread, time};
use rand::Rng;

const ROWS: usize = 20;
const COLS: usize = 100;

// Structure to represent a cell in the grid
#[derive(Clone, Copy)]
struct Cell {
    is_alive: bool,
    num_live_neighbors: usize,
}

fn main() {
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

            // Check the 8 cells surrounding the
