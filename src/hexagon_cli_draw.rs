use std::fmt::format;

use crate::hexagon_grid::HexagonGrid;


fn stage_1(a: bool) -> &'static str{
    if a{
        " _ _ "
    } else {
        "     "
    }
}
/* 
fn stage_2(b: bool){
    let x  = "/ \\";
}

fn stage_3(){
    let x = "/  \\__";
} */

pub fn draw(grid_obj: &HexagonGrid){
    let quad_grid = grid_obj.get_quad_grid();
    let (row_len, col_len) = grid_obj.get_quad_c_size();
    
  /*   for row in quad_grid{
        for maybe_block in row{
            match maybe_block {
                Some(block) => {
                    buffer = format!("{buffer} [{}, {}]", block.x, block.y)
                },
                None => {
                    buffer = format!("{buffer} [    ]")
                },
            }
        }
        buffer += "\n"
    } */
    let row_length_chars = if row_len % 2 == 0{
        row_len.div_ceil(2) * 14
    } else {
        row_len.div_ceil(2) * 14 + (9 * row_len % 2) + 9
    } + 2;
    let mut buffer = (" ".repeat(row_length_chars) + "\n").repeat((col_len * 4) + 1 + 2);

    let row_length_chars_with_endl = row_length_chars + 1;
    grid_obj.grid.iter().for_each(|block|{
        let (x, y) = block.get_quad_coords_xy();
        let is_pair = x % 2 == 0;
        let x_offset = ((7 * x) + 3) as usize + if is_pair{0}else{
            (row_length_chars_with_endl * 2)
        };
        let y_offset =  4 * y * row_length_chars_with_endl;
        let top_t = x_offset + y_offset;
        buffer.replace_range(top_t..top_t+3, "_ _");
        let bot_t = top_t + (4 * row_length_chars_with_endl);
        buffer.replace_range(bot_t..bot_t+3, "_ _");

        let top_left_t_1 = top_t + (row_length_chars_with_endl - 2);
        let top_left_t_2 = top_t + ((row_length_chars_with_endl * 2) - 3);
        buffer.replace_range(top_left_t_1..top_left_t_1+1, "/");
        buffer.replace_range(top_left_t_2..top_left_t_2+1, "/");

        let top_right_t_1 = top_t + (row_length_chars_with_endl + 4);
        let top_right_t_2 = top_t + ((row_length_chars_with_endl * 2) + 5);
        buffer.replace_range(top_right_t_1..top_right_t_1+1, "\\");
        buffer.replace_range(top_right_t_2..top_right_t_2+1, "\\");

        let bot_left_t_1 = top_t + (row_length_chars_with_endl * 3 - 3);
        let bot_left_t_2 = top_t + ((row_length_chars_with_endl * 4) - 2);
        buffer.replace_range(bot_left_t_1..bot_left_t_1+1, "\\");
        buffer.replace_range(bot_left_t_2..bot_left_t_2+1, "\\");

        let bot_right_t_1 = top_t + (row_length_chars_with_endl * 3 + 5);
        let bot_right_t_2 = top_t + ((row_length_chars_with_endl * 4) + 4);
        buffer.replace_range(bot_right_t_1..bot_right_t_1+1, "/");
        buffer.replace_range(bot_right_t_2..bot_right_t_2+1, "/"); 

        let text_1_t = top_t + (row_length_chars_with_endl * 1) - 1;
        buffer.replace_range(text_1_t..text_1_t+5, block.text[0]);

        let text_2_t = top_t + (row_length_chars_with_endl * 2) - 2;
        buffer.replace_range(text_2_t..text_2_t+7, block.text[1]);

        let text_3_t = top_t + (row_length_chars_with_endl * 3) - 2;
        buffer.replace_range(text_3_t..text_3_t+7, block.text[2]);
    });
    /* for y in 0..col_len{
        for x in 0..row_len{
        }
    } */
    println!("{buffer}");
  /*   

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
    
    
    for row_i in 0..lattices.col_len{
        for stage_i in 0..2{
            if row_i > 0 && row_i % lattices.row_len == 0{
                buffer += "\n"
            }
            let x = lattices.top[row_i];
            buffer += match stage_i {
                0 => stage_1(x),
                1 => {
                    ""
                }
                _ => panic!("unknown stage")
            }
        }
    } */
    /* lattices.top.iter().enumerate().for_each(|(lat_i,&lattice)|{
        if lat_i > 0 && lat_i % lattices.row_len == 0{
            buffer += "\n"
        }
        if lattice{
            buffer += 
        } else {
            buffer += "         "
        }
    });
    buffer += "\n";
    println!("{}", buffer); */
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
