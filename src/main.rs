
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

fn grid_test() -> Result<(), hexagons_in_cli::grid::HexagonGridError>{
    let mut grid = hexagons_in_cli::grid::HexagonGrid::new();
    grid.add_block_quad_c(0,0)?;
    grid.add_block_quad_c(0,0).expect_err("This shouldn't be adding a block at the same coords.");
    grid.add_block_hex_c(1, 1)?;
    grid.add_block_quad_c(1,0).expect_err("This shouldn't be adding a block at the same coords because translation system");

    //grid.add_block_hex_c(2, 2)?;
    grid.add_block(hexagons_in_cli::hexblock_q!(2,1, "2-2", "2-1"))?;
    grid.add_block_hex_c(3, 4)?;
    grid.add_block(hexagons_in_cli::hexblock_h!(2,1,"2-1", "2-0"))?
        .add_block(hexagons_in_cli::hexblock_h!(3,3, "aaaaaaaaaaaaaa", "4"))?;
    let neighbors  =
        grid
            .get_neighbors_of_with_hex_c(2, 2)
            .into_iter()
            .map(|x|{(x.x, x.y)})
            .collect::<Vec<(u16, u16)>>();
    
    assert_eq!(*neighbors, [(1,1),(2,1), (3,3)]);
    
    // width is 4, but starting from 1 not zero, (I know it's a bit upsetting)
    assert_eq!(grid.get_hex_c_size(), (4,5));
    //println!("{:?}", grid.get_quad_grid());
    let mut block = hexagons_in_cli::hexblock_q!(0,2, "12345", "1234567", "don't worry it's trimed automatically");
    block.text[2] = hexagons_in_cli::text::format_string("please worry it's NOT trimed automatically this way, you have to care for it yourself".to_string(), 7);
    grid.add_block(block)?;
    hexagons_in_cli::cli_draw::draw(&grid);
    Ok(())
}