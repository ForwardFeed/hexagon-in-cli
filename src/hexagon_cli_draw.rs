use crate::hexagon_grid::HexagonGrid;

struct HexagonLattices {
    row_len: usize,
    col_len: usize,
    top: Vec<bool>
}

pub fn draw(grid_obj: &HexagonGrid){
    let quad_grid = grid_obj.get_quad_grid();
    let (row_len, col_len) = grid_obj.get_quad_c_size();

    let mut lattices = HexagonLattices{
        row_len,
        col_len,
        top: vec![]
    };

    quad_grid.iter().for_each(|row|{
        row.iter().for_each(|col|{
            match col {
                Some(_) => {
                    lattices.top.push(true);
                },
                None => {
                    lattices.top.push(false);
                },
            };
            
        });
    });
    let mut buffer = String::new();
    lattices.top.iter().enumerate().for_each(|(lat_i,&lattice)|{
        if lat_i > 0 && lat_i % lattices.row_len == 0{
            buffer += "\n"
        }
        if lattice{
            buffer += "   _ _   "
        } else {
            buffer += "         "
        }
    });
    buffer += "\n";
    println!("{}", buffer);
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
    /* let grid = grid_obj.get_quad_grid();
    let (x,y) = grid_obj.get_quad_c_size();
    let mut buffer = String::new();
    for (row_i, row) in grid.iter().enumerate(){
        for (col_i, col) in row.iter().enumerate(){
            let block = match col {
                Some(x) => x,
                None => {
                    buffer += "        ";
                    continue;
                },
            };
            // the first row is a bit odd because unlike the rest it won't follow pattern
            /* if row_i == 0{
                
            }*/
            buffer += "   _ _   "
        }
        buffer += "\n"
    }
    println!("{buffer}, {grid:?}"); */
}
