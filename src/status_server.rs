use crate::module::ModuleMessage;
use crate::x11_utils::RootWindow;
use actix::prelude::*;

pub struct StatusServer {
    pub root_window: RootWindow,
    pub segments: Vec<String>,
    pub delimiter: &'static str,
}

impl Actor for StatusServer {
    type Context = Context<Self>;
}

impl Handler<ModuleMessage> for StatusServer {
    type Result = ();

    fn handle(&mut self, msg: ModuleMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let ModuleMessage(value, index) = msg; // disasemble the message
        std::mem::replace(&mut self.segments[index], value); //replace the value within the array
        self.update_bar(); // and write it to the bar
    }
}

impl StatusServer {
    fn update_bar(&self) {
        let content: String = self.segments.join(self.delimiter);
        self.root_window.update_bar(content);
    }

    pub fn new(amount_modules: usize, delimiter: &'static str) -> Self {
        Self {
            root_window: RootWindow::init_window(),
            segments: Vec::with_capacity(amount_modules),
            delimiter,
        }
    }
}
