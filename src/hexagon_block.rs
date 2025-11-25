

pub struct HexagonBlock{
    pub is_rendered: bool   
}


impl HexagonBlock {
    pub fn new() -> Self{
        HexagonBlock { is_rendered: false }
    }
    pub fn toggle(&mut self){
        self.is_rendered = ! self.is_rendered;
    }
}