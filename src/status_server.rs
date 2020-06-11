use crate::x11_utils::RootWindow;

pub struct StatusServer {
    pub root_window: RootWindow,
    pub content: Vec<String>,
}
