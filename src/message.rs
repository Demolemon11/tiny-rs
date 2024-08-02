use crate::state::log_text_state::LogText;

#[derive(Debug, Clone)]
pub enum Thing {
    APi(String),
    Path,
}

#[derive(Debug, Clone)]
pub enum ClearSth {
    APi,
    Path,
}

#[derive(Debug, Clone)]
pub enum Message {
    Add(Thing),
    ClearPath,
    Convert,
    ToggleButtonStyle,
    ToggleTheme,
    Display(LogText),
}
