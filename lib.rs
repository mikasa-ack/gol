#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod gol {

    #[ink(storage)]
    pub struct Gol {
        grid: Vec<Vec<bool>>,
    }

    impl Gol {
        #[ink(constructor)]
        pub fn new(init_grid: Vec<Vec<bool>>) -> Self {
            Self { grid: init_grid }
        }

        #[ink(message)]
        pub fn tick(&mut self) {}

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Vec<Vec<bool>> {
            self.grid.clone()
        }
    }
}
