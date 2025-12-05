use std::fmt::Debug;

use crate::{text::HexagonText, translate_coords::translate_hex_to_quad_coordinate};


// expected to use hex grid coordinate system
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
    pub fn get_quad_coords_xy(&self) -> (usize, usize) {
        let (x, y) = translate_hex_to_quad_coordinate(self.x, self.y);
        return (x as usize, y as usize);
    }
}


impl<'a> Debug for HexagonBlock<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HexagonBlock")
            .field("x", &self.x)
            .field("y", &self.y)
            //.field("text", &self.text)
            .finish()
    }
}