use crate::hexagon_grid::HexagonGrid;



pub fn draw(grid_obj: &HexagonGrid){
    let grid = grid_obj.get_quad_grid();
    grid.iter().for_each(|row|{
        row.iter().for_each(|col|{
            let block = match col {
                Some(x) => x,
                None => return,
            };
            
        })
    });
}


pub fn try_draw(){
    let buffer = String::new();
    
}