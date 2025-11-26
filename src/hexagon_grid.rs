use crate::hexagon_block::HexagonBlock;

use self::HexagonGridError::*;

type Coords = (u16, u16);

#[derive(Debug)]
pub enum HexagonGridError {
    CannotAddAlreadyPresent
}

pub struct HexagonGrid{
    grid: Vec<HexagonBlock>
}

// this always compute "odds downwards"
// this means that odd (non-pair) cols are downwards
pub fn translate_quad_to_hex_coordinate(x: u16, y:u16) -> Coords{
    (x ,u16::div_ceil(x, 2) + y)
}
pub fn translate_hex_to_quad_coordinate(x: u16, y:u16) -> Coords{
    (x ,y - u16::div_ceil(x, 2))
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