#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod gol {
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Gol {
        grid: Vec<Vec<bool>>,
        height: u64,
        width: u64,
        stop_at: u128,
        manual_stop: bool,
        manual_kill: bool,
    }
    impl Gol {
        /// Initializes the constructor with an initial grid.
        #[ink(constructor)]
        pub fn new(init_grid: Vec<Vec<bool>>) -> Self {
            Self {
                height: init_grid.len() as u64,
                width: init_grid[0].len() as u64,
                grid: init_grid,
                stop_at: 0,
                manual_stop: false,
                manual_kill: false,
            }
        }

        /// This function is called by the autonomous smart contract feature to update the state of the
        /// contract.
        /// # Note
        /// This function is only called if `should_execute` returns `true`.
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

        /// Returns the number of live neighbours around a given cell.
        /// # Arguments
        /// * `row` - The row of the cell to check.
        /// * `col` - The column of the cell to check.
        /// # Returns
        /// The number of live neighbours around the given cell.
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
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn set_stop_at(&mut self, value: u128) {
            self.stop_at = value;
        }

        fn get_alive_cells_count(&self) -> u128 {
            let mut count = 0;
            self.grid.iter().for_each(|line| {
                line.iter()
                    .for_each(|cell| count += if *cell { 1 } else { 0 })
            });
            count
        }

        /// Returns `true` if the autonomous call should be executed.
        #[ink(message)]
        pub fn should_execute(&self) -> bool {
            /*if self.manual_stop {
                return false;
            }
            self.get_alive_cells_count() == self.stop_at*/
            !self.manual_stop
        }

        /// Returns `true` if the autonomous call should be killed.
        #[ink(message)]
        pub fn should_kill(&self) -> bool {
            self.manual_kill
        }

        #[ink(message)]
        pub fn set_manual_stop(&mut self, value: bool) {
            self.manual_stop = value;
        }

        #[ink(message)]
        pub fn set_manual_kill(&mut self, value: bool) {
            self.manual_kill = value;
        }

        #[ink(message)]
        pub fn get_manual_stop(&self) -> bool {
            self.manual_stop
        }

        #[ink(message)]
        pub fn get_manual_kill(&self) -> bool {
            self.manual_kill
        }
    }
}
