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
    grid.add_block_quad_c(0,0).expect_err("This shouldn't be adding a block at the same coords.");
    grid.add_block_hex_c(1, 1)?;
    grid.add_block_quad_c(1,0).expect_err("This shouldn't be adding a block at the same coords especially with the translation system");

    grid.add_block_hex_c(2, 2)?;
    grid.add_block_hex_c(3, 4)?;
    grid.add_block_hex_c(2, 1)?;
    grid.add_block_hex_c(3, 3)?;
    // 1,1 has been added earlier
    let neighbors = grid.get_neighbors_of_with_hex_c(2, 2);
    assert_eq!(neighbors.len(), 3);

    Ok(())
}