# hexagon-in-cli
mediocre quality rust library that output hexagons into the CLI on an invisible grid with writeable insides

## This library just works for what I needed to

## How it works

Well it works with an internal grid system, which has a brilliant <small>[^citation needed¹]</small> hex grid coordinate system.
Which you can ignore if you use the quad coordinate all along, from input to ouput.

I believe that the hex system is mathematically more right, but it's mind warping to use it, so do yourself a favor and think with the quad system.
Look at he grid_example.txt to see what I mean.

It's also thought to be only used in a unsigned positive world, didn't planned to work in a signed coordinate world.

If you want to play with the library you can clone and play inside the main.rs, where I made a questionable test ground.
Speaking of testing, I didn't. It looked to work so I never tempted the eternal slumber of making tests
maybe with mind altering substances or money (same thing) in the future.

## How it works (for real this time)

```rs
// Make yourself a grid, as mentionned earlier those hexagons are to be 
// inside a G-R-I-D
let mut grid = hexagons_in_cli::grid::HexagonGrid::new();

// please do care that I use quad_c(oordinates here)
grid.add_block_quad_c(0,0)?;
// This will panic if you try to add twice the same one
grid.add_block_quad_c(0,0).expect_err("This shouldn't be adding a block at the same coords.");

// please do care that I use hex_c(oordinates here)
grid.add_block_hex_c(1, 1)?;
// as you can see translation seems to work
grid.add_block_quad_c(1,0).expect_err("This shouldn't be adding a block at the same coords because translation system");

// Oh yeah I mentionned that you could write into the hexagons
// I made some macros for that (hexblock_q for quad, hexblock_h for hex)
// Please notice that I use the _q(uad coordinate here)
let mut block = hexagons_in_cli::hexblock_q!(0,2, "12345", "1234567", &format!("don't worry it's trimed automatically"));
// about this block.text[2], I gotta explain how this text work
// there's 3 lines, hence [0] [1] [2], anything else and it will panic, also there's character limits to know if you modify that way
// [0] => 5 char max, [1] => 7 char max, [2] => 7 char max
// also format_string will automatically justify in the center, because it's much prettier than that
block.text[2] = hexagons_in_cli::text::format_string("please WORRY it's NOT trimed automatically this way, you have to care for it with that format_string function, or do it yourself".to_string(), 7);
// once added to the grid, it cannot be modified again.
// I know that sucks but if you enjoy doing lifetimes that's your problem not mine.
grid.add_block(block)?;
// then just call a draw and it will automatically print it, in dah terminal
hexagons_in_cli::cli_draw::draw(&grid);


//Honorable mentions:

// getting neighbors of an hex.
// actually the reason I kept using that hex coordinate system under the hood
// (I swear it's just barely mathematically better than just keeping the grid as an oddly shaped square made grid...)
grid.get_neighbors_of_with_hex_c(x,y);

// get the size of the grid
grid.get_quad_c_size() // or get_hex_c_size

// okay, so internally, I represented the grid into an array
// which I believed to be useful, because I originally wanted to draw on the terminal another way
// the way hexagons_in_cli::cli_draw::draw() works is that by making a buffer of spaces and endlines.
// Then iterating over the vector and add text where it has to be added.
// which works fine, but ultimately it's not perfect as I wished to do it iteratively and change the way I store data
grid.get_quad_grid()

// you can access the grid as a vector, you can read the wall of text up above to get an idea.
grid.grid
```

You may already feel it, but I was quite disapointed with myself with that work.
Because it took more time than it should, by pursuing fruitless optimization.
At least I published it this time.
Also there's probably an unspoken amount of bugs which I never cared to look for.
