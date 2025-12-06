use crate::{block::HexagonBlock, translate_coords::{translate_hex_to_quad_coordinate, translate_quad_to_hex_coordinate}};

use self::HexagonGridError::*;

pub type QuadGridRow<'a> = Vec<Option<&'a HexagonBlock>>;
pub type QuadGrid<'a> = Vec<QuadGridRow<'a>>;

#[derive(Debug)]
pub enum HexagonGridError {
    CannotAddAlreadyPresent
}

pub struct HexagonGrid{
    pub grid: Vec<HexagonBlock>,
    max_x: u16,
    max_y: u16,
    max_quad_y: u16,
}

impl HexagonGrid{
    pub fn new() -> Self{
        HexagonGrid { 
            grid: Vec::new(),
            max_x: 0,
            max_y: 0,
            max_quad_y: 0,
        }
    }

    pub fn get_hex_c_size(&self) -> (usize,usize){
        ((self.max_x + 1) as usize, (self.max_y + 1) as usize)
    }

    pub fn get_quad_c_size(&self) -> (usize,usize){
        ((self.max_x + 1) as usize, (self.max_quad_y + 1) as usize)
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
            let x_diff = u16::abs_diff(block.x, x);
            if x_diff > 1{
                return false;
            }
            let y_diff = u16::abs_diff(block.y, y);
            if y_diff > 1 {
                return false;
            }
            // find self
            if x_diff == 0 && y_diff == 0{
                return false;
            }
            return true;
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
                self.adapt_grid_size_tracker(x,y);
                Ok(())
            },
        }
    }

    fn adapt_grid_size_tracker(&mut self, x: u16, y:u16){
          if self.max_x < x{
            self.max_x = x
        }
        if self.max_y < y{
            self.max_y = y
        }
        let (_q_x, q_y) = translate_hex_to_quad_coordinate(x, y);
        if self.max_quad_y < q_y{
            self.max_quad_y = q_y
        }
    }
    
    pub fn add_block(&mut self, block: HexagonBlock) -> Result<&mut Self, HexagonGridError>{
        match self.find_block_with_hex_c(block.x,block.y) {
            true => {
                return Err(CannotAddAlreadyPresent)
            },
            false => {
                self.adapt_grid_size_tracker(block.x,block.y);
                self.grid.push(block);
            }
        }
        Ok(self)
    }
    
    pub fn add_block_quad_c(&mut self, x: u16, y:u16) -> Result<(), HexagonGridError>{
        let (tx, ty) = translate_quad_to_hex_coordinate(x, y);
        self.add_block_hex_c(tx, ty)
    }

    pub fn get_quad_grid(&self) -> QuadGrid<'_>{
        // vector initialization
        let mut row_handler: QuadGrid = (0..self.max_quad_y + 1).map(|_|{
            (0..self.max_x + 1).map(|_|{
                None
            }).collect::<QuadGridRow>()
        }).collect::<QuadGrid>();
        // vector feeding
        self.grid.iter().for_each(|hex|{
            let (x,y) = hex.get_quad_coords_xy();
            row_handler[y as usize][x as usize] = Some(hex);
        });
        row_handler
    }
    
}