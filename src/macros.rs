use crate::block::HexagonBlock;
use crate::text::HexagonText;

/**
 * Creates a hexagon block, with quad coordinate (hence the _q)
 */
macro_rules! hexblock_q {
    ($x:expr, $y:expr) => {
        HexagonBlock::new($x, $y)
    };
    ($x:expr, $y:expr, $line_0: tt) => {
        HexagonBlock{
            x: $x,
            y: $y,
            text: HexagonText::with_lines($line_0, "       ", "       ")
        }
    };
    ($x:expr, $y:expr, $line_0: tt, $line_1: tt) => {
        HexagonBlock{
            x: $x,
            y: $y,
            text: HexagonText::with_lines($line_0, $line_1, "       ")
        }
    };
    ($x:expr, $y:expr, $line_0: tt, $line_1: tt, $line_2: tt) => {
        HexagonBlock{
            x: $x,
            y: $y,
            text: HexagonText::with_lines($line_0, $line_1, $line_2: tt)
        }
    };
}
