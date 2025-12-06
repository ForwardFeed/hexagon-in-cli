/**
 * Creates a hexagon block, with hex coordinate (hence the _h)
 */
#[macro_export]
macro_rules! hexblock_h {
    ($x:expr, $y:expr) => {
        hexagons_in_cli::block::HexagonBlock::new($x, $y)
    };
    ($x:expr, $y:expr, $line_0: expr) => {
        hexagons_in_cli::block::HexagonBlock{
            x: $x,
            y: $y,
            text: hexagons_in_cli::text::HexagonText::with_lines($line_0, "       ".into(), "       ".into())
        }
    };
    ($x:expr, $y:expr, $line_0: expr, $line_1: expr) => {
        hexagons_in_cli::block::HexagonBlock{
            x: $x,
            y: $y,
            text: hexagons_in_cli::text::HexagonText::with_lines($line_0, $line_1, "       ".into())
        }
    };
    ($x:expr, $y:expr, $line_0: expr, $line_1: expr, $line_2: expr) => {
        hexagons_in_cli::block::HexagonBlock{
            x: $x,
            y: $y,
            text: hexagons_in_cli::text::HexagonText::with_lines($line_0, $line_1, $line_2)
        }
    };
}

/**
 * Creates a hexagon block, with quad coordinate (hence the _q)
 */
#[macro_export]
macro_rules! hexblock_q {
    ($x:expr, $y:expr) => {
        hexagons_in_cli::block::HexagonBlock::new($x, $y + u16::div_ceil($x, 2))
    };
    ($x:expr, $y:expr, $line_0: expr) => {
        hexagons_in_cli::block::HexagonBlock{
            x: $x,
            y: $y + u16::div_ceil($x, 2),
            text: hexagons_in_cli::text::HexagonText::with_lines($line_0, "       ".into(), "       ".into())
        }
    };
    ($x:expr, $y:expr, $line_0: expr, $line_1: expr) => {
        hexagons_in_cli::block::HexagonBlock{
            x: $x,
            y: $y + u16::div_ceil($x, 2),
            text: hexagons_in_cli::text::HexagonText::with_lines($line_0, $line_1, "       ".into())
        }
    };
    ($x:expr, $y:expr, $line_0: expr, $line_1: expr, $line_2: expr) => {
        hexagons_in_cli::block::HexagonBlock{
            x: $x,
            y: $y + u16::div_ceil($x, 2),
            text: hexagons_in_cli::text::HexagonText::with_lines($line_0, $line_1, $line_2)
        }
    };
}
