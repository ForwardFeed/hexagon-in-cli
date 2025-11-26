use hexagon_in_cli::hexagon_grid::HexagonGridError;


fn main(){
    //create_hexagon()
    match grid_test(){
        Ok(_) => {
            println!("grid test OK")
        },
        Err(x) => {
            println!("Error, Grid test: {x:?}")
        },
    }
}

fn grid_test() -> Result<(), HexagonGridError>{
    let mut grid = hexagon_in_cli::hexagon_grid::HexagonGrid::new();
    grid.add_block_quad_c(0,0)?;
    grid.add_block_quad_c(0,0).expect_err("This shouldn't be adding a block at the same place.");
    Ok(())
}