extern crate fixedbitset;
use fixedbitset::FixedBitSet;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// we dont need this since we are using fixed bit shit to save memory
// impl Cell {
//     fn toggle(&mut self) {
//         *self = match *self {
//             Cell::Dead => Cell::Alive,
//             Cell::Alive => Cell::Dead,
//         };
//     }
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            // self.cells[idx] = Cell::Alive;
            self.cells.set(idx, true);
        }
    }
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // Took me forever to figure out how to make this abstract to n x m universe sizes
    // fn get_single_glider_starting_configuration(width: u32, height: u32) -> Vec<Cell> {
    //     let first_cell_index = (height + 2) - height;
    //     let second_cell_index = height;
    //     let third_cell_index = height + 2;
    //     let fourth_cell_index = (height * 2) + 1;
    //     let fifth_cell_index = (height * 2) + 2;

    //     let cells = (0..width * height)
    //         .map(|i| {
    //             if i == first_cell_index
    //                 || i == second_cell_index
    //                 || i == third_cell_index
    //                 || i == fourth_cell_index
    //                 || i == fifth_cell_index
    //             {
    //                 Cell::Alive
    //             } else {
    //                 Cell::Dead
    //             }
    //         })
    //         .collect();

    //     return cells;
    // }

    // fn get_random_universe_configuration(width: u32, height: u32) -> Vec<Cell> {
    //     let cells = (0..width * height)
    //         .map(|i: u32| {
    //             let random_number = js_sys::Math::random();
    //             if random_number > 0.5 {
    //                 Cell::Alive
    //             } else {
    //                 Cell::Dead
    //             }
    //         })
    //         .collect();
    //     return cells;
    // }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 256;
        let height = 256;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
        // self.cells[idx]
    }

    pub fn spawn_glider(&mut self, row: u32, col: u32) {
        let glider = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];

        for (r, c) in glider.iter() {
            let row = (row + r) % self.height;
            let col = (col + c) % self.width;
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    pub fn spawn_pulsar(&mut self, row: u32, col: u32) {
        let pulsar = [
            (2, 0),
            (3, 0),
            (4, 0),
            (8, 0),
            (9, 0),
            (10, 0),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 8),
            (0, 9),
            (0, 10),
            (5, 2),
            (5, 3),
            (5, 4),
            (5, 8),
            (5, 9),
            (5, 10),
            (7, 2),
            (7, 3),
            (7, 4),
            (7, 8),
            (7, 9),
            (7, 10),
            (12, 2),
            (12, 3),
            (12, 4),
            (12, 8),
            (12, 9),
            (12, 10),
            (2, 5),
            (3, 5),
            (4, 5),
            (8, 5),
            (9, 5),
            (10, 5),
            (2, 7),
            (3, 7),
            (4, 7),
            (8, 7),
            (9, 7),
            (10, 7),
            (2, 12),
            (3, 12),
            (4, 12),
            (8, 12),
            (9, 12),
            (10, 12),
            (12, 5),
            (12, 7),
        ];

        for (r, c) in pulsar.iter() {
            let row = (row + r) % self.height;
            let col = (col + c) % self.width;
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }
}
