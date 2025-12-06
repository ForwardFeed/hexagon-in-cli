use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct HexagonText{
    line_0: String,
    line_1: String,
    line_2: String,
}

impl  HexagonText {
    pub fn new() -> Self {
        HexagonText{
            line_0: "     ".to_string(),
            line_1: "       ".to_string(),
            line_2: "       ".to_string(),
        }
    }
    pub fn with_lines(line_0: &str, line_1: &str, line_2: &str) -> Self{
        HexagonText {
            line_0: format_string(line_0.to_string(), 5),
            line_1: format_string(line_1.to_string(),7),
            line_2: format_string(line_2.to_string(), 7) 
        }
    }
}


impl Index<usize> for HexagonText{
    type Output = String;

    fn index(&self, index: usize) -> &String {
        match index {
            0 => &self.line_0,
            1 => &self.line_1,
            2 => &self.line_2,
            _ => panic!("unknown index: {index}")
        }
    }
}

impl <'a> IndexMut<usize> for HexagonText{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.line_0,
            1 => &mut self.line_1,
            2 => &mut self.line_2,
            _ => panic!("unknown index: {index}")
        }
    }
}


pub fn format_string(str: String, max_len: usize) -> String{
    match str.len(){
        len if len == max_len=> {
            str
        }
        len if len > max_len =>{
            str[..max_len].to_string()
        }
        len =>{
            let missing_len = max_len - len;
            if missing_len % 2 == 0{
                let padding_n = missing_len.div_ceil(2);
                format!("{}{}{}", " ".repeat(padding_n), str.clone(), " ".repeat(padding_n))
            } else {
                let padding_left = missing_len.div_ceil(2);
                format!("{}{}{}", " ".repeat(padding_left - 1), str, " ".repeat(padding_left))
            }
            
        }
    }
}