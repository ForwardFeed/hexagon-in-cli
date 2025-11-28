use crate::hexagon_grid::HexagonGrid;



pub fn draw(grid_obj: &HexagonGrid){
    /* let grid = grid_obj.get_quad_grid();
    grid.iter().for_each(|row|{
        row.iter().for_each(|col|{
            let block = match col {
                Some(x) => x,
                None => return,
            };
            block.text[1];
            
        })
    }); */
    let grid = grid_obj.get_quad_grid();
    let (x,y) = grid_obj.get_quad_c_size();
    let mut buffer = String::new();
    for (row_i, row) in grid.iter().enumerate(){
        for (col_i, col) in row.iter().enumerate(){
            let block = match col {
                Some(x) => x,
                None => continue,
            };
            // the first row is a bit odd because unlike the rest it won't follow pattern
            if row_i == 0{
                buffer += "   _ _   "
            } else {
                buffer += "        "
            }
        } 
    }
    println!("{buffer}, {grid:?}");
}
