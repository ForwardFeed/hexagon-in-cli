/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */

use crate::hexagon_state::HexagonDrawState;

mod hexagon_state;

// width: 9
const RAW_HEXAGON: [&str; 5] = [
    "   _ _   ",
    " /     \\ ",
    "/       \\",
    "\\       /",
    " \\ _ _ / ",
];

pub fn create_hexagon(){
    let hexagon_list = [
        hexagon_state::HexagonDrawState::new(),
        HexagonDrawState::with_params(true, false, false, false, false, false),
        HexagonDrawState::with_params(false, true, false, false, false, false),
        HexagonDrawState::with_params(false, false, true, false, false, false),
        HexagonDrawState::with_params(false, false, false, true, false, false),
        HexagonDrawState::with_params(false, false, false, false, true, false),
        HexagonDrawState::with_params(false, false, false, false, false, true),
    ];
    let mut textbuffer = String::new();
    for hexagon in hexagon_list{
        textbuffer += format!("{:?}\n", hexagon).as_str();
        for j in 0..5 {
            let addition = match j {
                0 => {
                    if hexagon.top{
                        "   _ _   "
                    } else {
                        "         "
                    }
                },
                1 => {
                    if hexagon.top_right{
                        if hexagon.top_left{
                            " /     \\ "
                        } else {
                            " /       "
                        }
                    } else {
                        if hexagon.top_left{
                            "       \\ "
                        } else {
                            "         "
                        }
                    }
                },
                2 => {
                    if hexagon.top_right{
                        if hexagon.top_left{
                            "/       \\"
                        } else {
                            "/        "
                        }
                    } else {
                        if hexagon.top_left{
                            "        \\"
                        } else {
                            "         "
                        }
                    }
                },
                3 => {
                    if hexagon.bot_right{
                        if hexagon.bot_left{
                            "\\       /"
                        } else {
                            "\\        "
                        }
                    } else {
                        if hexagon.bot_left{
                            "        /"
                        } else {
                            "         "
                        }
                    }
                },
                _ => {
                    if hexagon.bot_right{
                        if hexagon.bot_left{
                            if hexagon.bot{
                                " \\ _ _ / "
                            } else {
                                " \\     / "
                            }
                        } else {
                            if hexagon.bot{
                                " \\ _ _   "
                            } else {
                                " \\       "
                            }
                        }
                    } else {
                        if hexagon.bot_left{
                            if hexagon.bot{
                                "   _ _ / "
                            } else {
                                "       / "
                            }
                        } else {
                            if hexagon.bot{
                                "   _ _   "
                            } else {
                                "         "
                            }
                        }
                    }
                },
            };
            textbuffer += addition;
            textbuffer += "\n";
        }
    }
    println!("{textbuffer}");
}