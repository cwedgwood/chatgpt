#include <iostream>
#include <vector>

using namespace std;

const int ROWS = 20;
const int COLS = 20;

// Structure to represent a cell in the grid
struct Cell {
  bool isAlive;
  int numLiveNeighbors;
};

// Function prototypes
void initializeGrid(vector<vector<Cell>> &grid);
void printGrid(const vector<vector<Cell>> &grid);
void updateGrid(vector<vector<Cell>> &grid);

int main() {
  // Create the grid of cells
  vector<vector<Cell>> grid(ROWS, vector<Cell>(COLS));

  // Initialize the grid
  initializeGrid(grid);

  // Print the initial grid
  cout << "Initial grid:" << endl;
  printGrid(grid);

  // Update and print the grid for 10 iterations
  for (int i = 0; i < 10; i++) {
    updateGrid(grid);
    cout << "Updated grid:" << endl;
    printGrid(grid);
  }

  return 0;
}

// Initializes the grid with random values
void initializeGrid(vector<vector<Cell>> &grid) {
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      grid[i][j].isAlive = rand() % 2;
      grid[i][j].numLiveNeighbors = 0;
    }
  }
}

// Prints the grid to the console
void printGrid(const vector<vector<Cell>> &grid) {
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      cout << (grid[i][j].isAlive ? 'X' : '.');
    }
    cout << endl;
  }
  cout << endl;
}

// Updates the grid according to the rules of The Game of Life
void updateGrid(vector<vector<Cell>> &grid) {
  // First, update the numLiveNeighbors field for each cell
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      int numLiveNeighbors = 0;

      // Check the 8 cells surrounding the current cell
      for (int k = -1; k <= 1; k++) {
        for (int l = -1; l <= 1; l++) {
          // Skip the current cell
          if (k == 0 && l == 0) continue;

          // Check if the neighboring cell is within the grid boundaries
          if (i + k >= 0 && i + k < ROWS && j + l >= 0 && j + l < COLS) {
            if (grid[i + k][j + l].isAlive) numLiveNeighbors++;
          }
        }
      }

      grid[i][j].numLiveNeighbors = numLiveNeighbors;
    }
  }

  // Next, update the state of each cell based on the number of live neighbors
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      int numLiveNeighbors = grid[i][j].numLiveNeighbors;

      // A live cell with fewer than 2 live neighbors dies (underpopulation)
      if (grid[i][j].isAlive && numLiveNeighbors < 2) {
        grid[i][j].isAlive = false;
      }
      // A live cell with 2 or 3 live neighbors lives on to the next generation
      else if (grid[i][j].isAlive && (numLiveNeighbors == 2 || numLiveNeighbors == 3)) {
        grid[i][j].isAlive = true;
      }
      // A live cell with more than 3 live neighbors dies (overpopulation)
      else if (grid[i][j].isAlive && numLiveNeighbors > 3) {
        grid[i][j].isAlive = false;
      }
      // A dead cell with exactly 3 live neighbors becomes a live cell (reproduction)
      else if (!grid[i][j].isAlive && numLiveNeighbors == 3) {
        grid[i][j].isAlive = true;
      }
    }
  }
}
