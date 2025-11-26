use crate::{hexagon_text::HexagonText, translate_coords::translate_hex_to_quad_coordinate};


// expected to use hex grid coordinate system
// any translation doesn't come from here
#[derive(Debug)]
pub struct HexagonBlock<'a>{
    pub x: u16,
    pub y: u16,
    pub text: HexagonText<'a>
}


impl <'a>HexagonBlock<'a> {
    pub fn new(x: u16, y:u16) -> Self{
        HexagonBlock { x, y, text: HexagonText::new() }
    }
    pub fn new_with_text(x: u16, y:u16, text: HexagonText<'a>) -> Self{
        HexagonBlock { x, y, text }
    }
    pub fn get_quad_coords_xy(&self) -> (u16, u16) {
        translate_hex_to_quad_coordinate(self.x, self.y)
    }
}