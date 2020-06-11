use crate::module::*;
use crate::status_server::StatusServer;
use crate::PauseBetweenYields;
use actix::prelude::*;
use std::boxed::Box;

/// A worker is spun up for each module to ensure that they
/// don't block each other
pub struct ModuleWorker {
    pub pause: PauseBetweenYields,
    pub module: Box<dyn Module>,
    pub status_server_addr: Addr<StatusServer>,
    pub index: usize,
}

impl ModuleWorker {
    pub fn new(
        wrapper: ModuleWrapper,
        status_server_addr: Addr<StatusServer>,
        index: usize,
    ) -> Self {
        let ModuleWrapper(module, pause) = wrapper;
        Self {
            pause,
            module,
            status_server_addr,
            index,
        }
    }
}

impl Actor for ModuleWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut SyncContext<Self>) {
        self.run();
    }
}

impl ModuleWorker {
    fn run(&mut self) {
        loop {
            let result = self.module.yield_next_value(); //obtain next value
            self.status_server_addr
                .do_send(ModuleMessage(result, self.index)); // send message to server to update bar
            std::thread::sleep(self.pause); // sleep until the duration has passed
        }
    }
}
