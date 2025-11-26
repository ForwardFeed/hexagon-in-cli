// this always compute "odds downwards"
// this means that odd (non-pair) cols are downwards
pub fn translate_quad_to_hex_coordinate(x: u16, y:u16) -> (u16, u16){
    (x ,u16::div_ceil(x, 2) + y)
}
pub fn translate_hex_to_quad_coordinate(x: u16, y:u16) -> (u16, u16){
    (x ,y - u16::div_ceil(x, 2))
}