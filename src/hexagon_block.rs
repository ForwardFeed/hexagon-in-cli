use crate::translate_coords::translate_hex_to_quad_coordinate;


// expected to use hex grid coordinate system
// any translation doesn't come from here
pub struct HexagonBlock{
    pub x: u16,
    pub y: u16
}


impl HexagonBlock {
    pub fn new(x: u16, y:u16) -> Self{
        HexagonBlock { x, y }
    }
    pub fn get_coords_xy(&self) -> (u16, u16) {
        translate_hex_to_quad_coordinate(self.x, self.y)
    }
}