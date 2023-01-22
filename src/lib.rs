mod utils;

use wasm_bindgen::prelude::*;

use std::fmt;

extern crate web_sys;

macro_rules! log {
    ( $($ t:tt)* ) => {
        web_sys::console::log_1(&format!($( $t )* ).into());
    };
}

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

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}


impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1,0,1].iter().cloned() {
            for delta_column in [self.width - 1,0,1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_column) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7  == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
               
            })
            .collect();

        Universe {
            width, 
            height, 
            cells
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                /*log!(
                    "cell [{}, {}] is initially {:?} and has {} live neighbors",
                    row, 
                    col, 
                    cell,
                    live_neighbors
                );*/

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive, 
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                //log!("      it becomes {:?}", next_cell);

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        return self.width
    }

    pub fn height(&self) -> u32 {
        return self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect()
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * self.height).map(|_i| Cell::Dead).collect()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn create_glider(&mut self, row: u32, column: u32) {
        let up = (row - 1) % self.height;
        let down = (row + 1) % self.height;
        let left = (column - 1) % self.width;
        let right = (column + 1) % self.width;
        self.set_cells(&[(up, left), (up, column), (row, column), (row, right), (down, left)]);
    }

    pub fn create_spaceship(&mut self, row: u32, col: u32) {
        let rm2 = (row - 2) % self.height;
        let rm1 = (row - 1) % self.height;
        let rp1 = (row + 1) % self.height;

        let cm2 = (col - 2) % self.width;
        let cm1 = (col - 1) % self.width;
        let cp1 = (col + 1) % self.width;
        let cp2 = (col + 2) % self.width;
        self.set_cells(&[
            (rm2, cm1),
            (rm2, col),
            (rm2, cp1),
            (rm2, cp2),
            (rm1, cm2),
            (rm1, cp2),
            (row, cp2),
            (rp1, cm2),
            (rp1, cp1),
        ]);
    }

    pub fn clean(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|_i| Cell::Dead)
            .collect();

    }
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}