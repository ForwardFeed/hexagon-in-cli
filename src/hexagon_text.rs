use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct HexagonText<'a>{
    line_0: &'a str,
    line_1: &'a str,
    line_2: &'a str,
}

impl<'a>  HexagonText<'a> {
    pub fn new() -> Self {
        HexagonText{
            line_0: "12345",
            line_1: "abcdefg",
            line_2: "1234567",
        }
    }
    pub fn with_lines(line_0: &'a str, line_1: &'a str, line_2: &'a str) -> Self{
        HexagonText { line_0, line_1, line_2 }
    }
}


impl <'a> Index<usize> for HexagonText<'a>{
    type Output = &'a str;

    fn index(&self, index: usize) -> &&'a str {
        match index {
            0 => &self.line_0,
            1 => &self.line_1,
            2 => &self.line_2,
            _ => panic!("unknown index: {index}")
        }
    }
}

impl <'a> IndexMut<usize> for HexagonText<'a>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.line_0,
            1 => &mut self.line_1,
            2 => &mut self.line_2,
            _ => panic!("unknown index: {index}")
        }
    }
}
