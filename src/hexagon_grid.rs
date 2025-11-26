use crate::{hexagon_block::HexagonBlock, translate_coords::translate_quad_to_hex_coordinate};

use self::HexagonGridError::*;

#[derive(Debug)]
pub enum HexagonGridError {
    CannotAddAlreadyPresent
}

pub struct HexagonGrid{
    grid: Vec<HexagonBlock>,
    max_x: u16,
    max_y: u16
}

impl HexagonGrid{
    pub fn new() -> Self{
        HexagonGrid { 
            grid: Vec::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn get_hex_c_size(&self) -> (u16,u16){
        (self.max_x, self.max_y)
    }

    fn find_block_with_hex_c(&self, x: u16, y:u16) -> bool{
        match self.grid.iter().find(|block|{
            block.x == x && block.y == y
        }){
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_neighbors_of_with_hex_c(&self, x: u16, y:u16) -> Vec<&HexagonBlock>{
        self.grid.iter().filter(|block|{
            let diff = u16::abs_diff(block.x, x) + u16::abs_diff(block.y, y);
            match diff{
                0 => false,// because it found it self
                1 => true,
                2 => true,
                _ => false
            }
        }).collect::<Vec<&HexagonBlock>>()
    }

    pub fn get_neighbors_of_with_quad_c(&mut self, x: u16, y:u16) -> Vec<&HexagonBlock>{
        let (tx, ty) = translate_quad_to_hex_coordinate(x, y);
        self.get_neighbors_of_with_hex_c(tx, ty)
    }

    pub fn add_block_hex_c(&mut self, x: u16, y:u16) -> Result<(), HexagonGridError> {
        match self.find_block_with_hex_c(x,y) {
            true => {
                Err(CannotAddAlreadyPresent)
            },
            false => {
                self.grid.push(HexagonBlock::new(x, y));
                // modify if must, the grid size tracker
                if self.max_x < x{
                    self.max_x = x
                }
                if self.max_y < y{
                    self.max_y = y
                }
                Ok(())
            },
        }
    }

    pub fn add_block_quad_c(&mut self, x: u16, y:u16) -> Result<(), HexagonGridError>{
        let (tx, ty) = translate_quad_to_hex_coordinate(x, y);
        self.add_block_hex_c(tx, ty)
    }

    
}