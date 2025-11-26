use crate::{hexagon_block::HexagonBlock, translate_coords::translate_quad_to_hex_coordinate};

use self::HexagonGridError::*;

#[derive(Debug)]
pub enum HexagonGridError {
    CannotAddAlreadyPresent
}

pub struct HexagonGrid{
    grid: Vec<HexagonBlock>
}

impl HexagonGrid{
    pub fn new() -> Self{
        HexagonGrid { 
            grid: Vec::new()
        }
    }

    fn find_block_with_hex_c(&self, x: u16, y:u16) -> bool{
        match self.grid.iter().find(|block|{
            block.x == x && block.y == y
        }){
            Some(_) => true,
            None => false,
        }
    }

    pub fn find_neighbors_of_with_hex_c(&self, x: u16, y:u16) -> Vec<&HexagonBlock>{
        self.grid.iter().filter(|block|{
            u16::abs_diff(block.x, x) + u16::abs_diff(block.y, y) <= 2
        }).collect::<Vec<&HexagonBlock>>()
    }

    pub fn find_neighbors_of_with_quad_c(&mut self, x: u16, y:u16) -> Vec<&HexagonBlock>{
        let (tx, ty) = translate_quad_to_hex_coordinate(x, y);
        self.find_neighbors_of_with_hex_c(tx, ty)
    }

    pub fn add_block_hex_c(&mut self, x: u16, y:u16) -> Result<(), HexagonGridError> {
        match self.find_block_with_hex_c(x,y) {
            true => {
                Err(CannotAddAlreadyPresent)
            },
            false => {
                self.grid.push(HexagonBlock::new(x, y));
                Ok(())
            },
        }
    }

    pub fn add_block_quad_c(&mut self, x: u16, y:u16) -> Result<(), HexagonGridError>{
        let (tx, ty) = translate_quad_to_hex_coordinate(x, y);
        self.add_block_hex_c(tx, ty)
    }

    
}