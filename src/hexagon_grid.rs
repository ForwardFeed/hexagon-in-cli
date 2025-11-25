

pub struct HexagonGrid{
    auto_translate_quad: bool,
    max_x: u16,
    max_y: u16,
}


impl HexagonGrid{
    pub fn new(auto_translate_from_quad: bool) -> Self{
        HexagonGrid { 
            auto_translate_quad: auto_translate_from_quad,
            max_x: 0,
            max_y: 0
        }
    }
}