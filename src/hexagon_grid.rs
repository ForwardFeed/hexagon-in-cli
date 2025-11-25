
type Coords = (u16, u16);

pub struct HexagonGrid{
    auto_translate_quad: bool,
    max_x: u16,
    max_y: u16,
    grid: Vec<Vec<bool>>
}


impl HexagonGrid{
    pub fn new(auto_translate_coord_from_quad: bool) -> Self{
        HexagonGrid { 
            auto_translate_quad: auto_translate_coord_from_quad,
            max_x: 0,
            max_y: 0,
            grid: vec![vec![]],
        }
    }
    fn translate_quad_to_hex_coordinate(x: u16, y:u16) -> Coords{
        
        (0,0)
    }
    fn translate_hex_to_quad_coordinate(x: u16, y:u16) -> Coords{
        
        (0,0)
    }
}