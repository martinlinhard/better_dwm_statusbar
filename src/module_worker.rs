use crate::messages::*;
use crate::module::Module;
use crate::status_server::StatusServer;
use actix::prelude::*;
use std::boxed::Box;
use std::collections::HashMap;

/// A worker is spun up for each module to ensure that they
/// don't block each other
pub struct ModuleWorker {
    pub modules: HashMap<usize, Box<dyn Module>>,
    pub status_server_addr: Addr<StatusServer>,
}

impl ModuleWorker {
    pub fn new(status_server_addr: Addr<StatusServer>) -> Self {
        Self {
            status_server_addr,
            modules: HashMap::new(),
        }
    }
}

impl Actor for ModuleWorker {
    type Context = SyncContext<Self>;
}

impl ModuleWorker {
    pub fn add_module(&mut self, index: usize, module: Box<dyn Module>) {
        self.modules.insert(index, module);
    }
}

impl Handler<ClockMessage> for ModuleWorker {
    type Result = ();
    fn handle(&mut self, msg: ClockMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        // Get the module which the clock specified
        let module = self.modules.get_mut(&msg.0).unwrap();
        // Get its content
        let content = module.yield_next_value();
        // And send it to the status server
        self.status_server_addr
            .do_send(ModuleMessage(content, msg.0));
    }
}
