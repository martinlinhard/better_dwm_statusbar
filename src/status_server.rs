use crate::x11_utils::RootWindow;
use actix::prelude::*;

pub struct StatusServer {
    pub root_window: RootWindow,
    pub content: Vec<String>,
}

impl Actor for StatusServer {
    type Context = Context<Self>;
}
