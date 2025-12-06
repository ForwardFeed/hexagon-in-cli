/**
 * Creates a hexagon block, with hex coordinate (hence the _h)
 */
#[macro_export]
macro_rules! hexblock_h {
    ($x:expr, $y:expr) => {
        hexagon_in_cli::block::HexagonBlock::new($x, $y)
    };
    ($x:expr, $y:expr, $line_0: tt) => {
        hexagon_in_cli::block::HexagonBlock{
            x: $x,
            y: $y,
            text: hexagon_in_cli::text::HexagonText::with_lines($line_0, "       ", "       ")
        }
    };
    ($x:expr, $y:expr, $line_0: tt, $line_1: tt) => {
        hexagon_in_cli::block::HexagonBlock{
            x: $x,
            y: $y,
            text: hexagon_in_cli::text::HexagonText::with_lines($line_0, $line_1, "       ")
        }
    };
    ($x:expr, $y:expr, $line_0: tt, $line_1: tt, $line_2: tt) => {
        hexagon_in_cli::block::HexagonBlock{
            x: $x,
            y: $y,
            text: hexagon_in_cli::text::HexagonText::with_lines($line_0, $line_1, $line_2: tt)
        }
    };
}

/**
 * Creates a hexagon block, with quad coordinate (hence the _q)
 */
#[macro_export]
macro_rules! hexblock_q {
    ($x:expr, $y:expr) => {
        hexagon_in_cli::block::HexagonBlock::new($x, $y + u16::div_ceil($x, 2))
    };
    ($x:expr, $y:expr, $line_0: tt) => {
        hexagon_in_cli::block::HexagonBlock{
            x: $x,
            y: $y + u16::div_ceil($x, 2),
            text: hexagon_in_cli::text::HexagonText::with_lines($line_0, "       ", "       ")
        }
    };
    ($x:expr, $y:expr, $line_0: tt, $line_1: tt) => {
        hexagon_in_cli::block::HexagonBlock{
            x: $x,
            y: $y + u16::div_ceil($x, 2),
            text: hexagon_in_cli::text::HexagonText::with_lines($line_0, $line_1, "       ")
        }
    };
    ($x:expr, $y:expr, $line_0: tt, $line_1: tt, $line_2: tt) => {
        hexagon_in_cli::block::HexagonBlock{
            x: $x,
            y: $y + u16::div_ceil($x, 2),
            text: hexagon_in_cli::text::HexagonText::with_lines($line_0, $line_1, $line_2: tt)
        }
    };
}
