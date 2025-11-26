
#[derive(Debug)]
pub struct HexagonText<'a>{
    line_0: &'a str,
    line_1: &'a str,
    line_2: &'a str,
}


impl<'a>  HexagonText<'a> {
    pub fn new() -> Self {
        HexagonText{
            line_0: "     ",
            line_1: "       ",
            line_2: "       ",
        }
    }
    pub fn with_lines(line_0: &'a str, line_1: &'a str, line_2: &'a str) -> Self{
        HexagonText { line_0, line_1, line_2 }
    }
}