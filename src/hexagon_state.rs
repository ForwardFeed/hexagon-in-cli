use std::fmt::Debug;



pub struct HexagonDrawState {
    pub top: bool,
    pub top_right: bool,
    pub top_left: bool,
    pub bot: bool,
    pub bot_right: bool,
    pub bot_left: bool,
}

impl HexagonDrawState {
    pub fn new() -> Self{
        return HexagonDrawState { 
            top: false,
            top_right: false,
            top_left: false,
            bot: false,
            bot_right: false,
            bot_left: false 
        }
    }
    pub fn with_params(
        top: bool,
        top_right: bool,
        top_left: bool,
        bot: bool,
        bot_right: bool,
        bot_left: bool,
    ) -> Self{
        return HexagonDrawState { 
            top,
            top_right,
            top_left,
            bot,
            bot_right,
            bot_left 
        }
    }
}

impl Debug for HexagonDrawState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_field = f.debug_struct("HexagonDrawState");
        macro_rules! s {
            ($text:tt, $t:ident) => {
                if self.$t{
                    debug_field.field($text, &self.$t);
                }
            };
        }
        s!("top", top);
        s!("top_right", top_right);
        s!("top_left", top_left);
        s!("bot", bot);
        s!("bot_right", bot_right);
        s!("bot_left", bot_left);
        debug_field.finish()
    }
}