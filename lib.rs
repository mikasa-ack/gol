#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod gol {
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Gol {
        grid: Vec<Vec<bool>>,
        height: u64,
        width: u64,
    }
    impl Gol {
        #[ink(constructor)]
        pub fn new(init_grid: Vec<Vec<bool>>) -> Self {
            Self {
                height: init_grid.len() as u64,
                width: init_grid[0].len() as u64,
                grid: init_grid,
            }
        }

        #[ink(message)]
        pub fn tick(&mut self) {
            let mut next_grid = self.grid.clone();
            for row in 0..self.grid.len() {
                for col in 0..self.grid[0].len() {
                    let next_cell = match (
                        self.grid[row][col],
                        self.get_alive_neighbours(row as u64, col as u64),
                    ) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (true, x) if x < 2 => false,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (false, 3) => true,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };
                    next_grid[row][col] = next_cell;
                }
            }
            self.grid = next_grid;
        }

        pub fn get_alive_neighbours(&self, row: u64, col: u64) -> u8 {
            let mut count = 0;
            for delta_row in [self.height - 1, 0, 1].iter().cloned() {
                for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                    if delta_row == 0 && delta_col == 0 {
                        continue;
                    }

                    let neighbor_row = (row + delta_row) % self.height;
                    let neighbor_col = (col + delta_col) % self.width;
                    count += self.grid[neighbor_row as usize][neighbor_col as usize] as u8;
                }
            }
            count
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Vec<Vec<bool>> {
            self.grid.clone()
        }

        #[ink(message)]
        pub fn should_execute(&self) -> bool {
            true
        }

        #[ink(message)]
        pub fn should_kill(&self) -> bool {
            false
        }
    }
}
