#[cfg(test)]
mod universe_tests;

use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(pub u32, pub u32, pub u32);

#[wasm_bindgen]
impl Pos {
    pub fn new(x: u32, y: u32, z: u32) -> Pos {
        Pos(x, y, z)
    }
}

#[wasm_bindgen]
pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    cells: Vec<Cell>,
    spec_cells: Vec<Pos>,
}


#[wasm_bindgen]
impl Universe {
    fn get_index(&self, pos: Pos) -> usize {
        let Pos(x, y, z) = pos;
        (x + y * self.width + z * self.width * self.height) as usize
    }

    fn set_spec_cell(&mut self, pos: Pos) {
        self.spec_cells.push(pos);
    }

    pub fn live_neighbor_count(&self, pos: Pos) -> u8 {
        let Pos(x, y, z) = pos;
        let mut count = 0_u8;
        for delta_x in [self.width - 1, 0, 1].iter().copied() {
            for delta_y in [self.height - 1, 0, 1].iter().copied() {
                for delta_z in [self.depth - 1, 0, 1].iter().copied() {
                    if delta_x == 0 && delta_y == 0 && delta_z == 0 {
                        continue;
                    }
                    let pos = Pos(
                        (x + delta_x) % self.width,
                        (y + delta_y) % self.height,
                        (z + delta_z) % self.depth,
                    );
                    match self.cells[self.get_index(pos)] {
                        Cell::Alive => {
                            count += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        count
    }
    pub fn new(sizes: Option<Pos>) -> Universe {
        let Pos(width, height, depth) = match sizes {
            Some(sizes) => sizes,
            None => Pos(10, 10, 10)
        };
        let cells = (0..width * height * depth).map(|_| Cell::Dead).collect();
        Universe { width, height, depth, cells, spec_cells: vec![] }
    }
    pub fn set_by_step(&mut self, step: usize) {
        self.cells = self.cells.iter().copied().enumerate().map(|(i, _)| {
            if i % step == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();
    }
    pub fn set_cell(&mut self, pos: Pos) -> Result<(), String> {
        let ind = self.get_index(pos);
        if ind < self.cells.len() {
            self.cells[ind] = match self.cells[ind] {
                Cell::Alive => Cell::Dead,
                Cell::Dead => Cell::Alive
            }
        } else {
            return Err("не возможно создать клетку вне поля".to_string());
        }
        Ok(())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn depth(&self) -> u32 {
        self.depth
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn get_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                for z in 0..self.depth {
                    let pos = Pos(x, y, z);
                    let cell = self.cells[self.get_index(pos)];
                    let live_neighbors = self.live_neighbor_count(pos);
                    let id = self.get_index(pos);
                    next[id] = match (cell, live_neighbors) {
                        (Cell::Alive, x) if x == 2 || x == 3 => Cell::Alive,
                        (Cell::Dead, x) if x == 3 => Cell::Alive,
                        (Cell::Alive, x) if x > 3 || x < 2 => Cell::Dead,
                        (other, _) => other
                    };
                }
            }
        }
        self.cells = next;
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line = (0..self.width * 2 + 2 + 8).map(|_| '-').collect::<String>();
        writeln!(f, "Матрица игры {}x{}x{}", self.width, self.height, self.depth)?;
        writeln!(f, "{}", line)?;
        println!("{:?}", self.spec_cells);
        for z in 0..self.depth {
            for y in 0..self.height {
                write!(f, "| ")?;
                for x in 0..self.width {
                    let pos = Pos(x, y, z);
                    let spec = self.spec_cells.iter().find(|&&x| x == pos);
                    let cell = self.cells[self.get_index(pos)];
                    write!(f, "{} ", match (spec, cell) {
                        (Some(_), Cell::Alive) => "☑",
                        (Some(_), Cell::Dead, ) => "✓",
                        (None, Cell::Alive, ) => "□",
                        (None, Cell::Dead, ) => "■",
                    })?;
                }
                if y == 0 {
                    writeln!(f, "| слой {}", z + 1)?;
                } else {
                    writeln!(f, "|")?;
                }
            }
            if z != self.depth - 1 {
                writeln!(f, "{}", line)?
            }
        }
        writeln!(f, "{}", (0..self.width * 2 + 3).map(|_| '-').collect::<String>())
    }
}