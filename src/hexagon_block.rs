
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
}