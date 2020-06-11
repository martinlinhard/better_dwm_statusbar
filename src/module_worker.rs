use crate::PauseBetweenYields;
use crate::module::*;
use crate::status_server::StatusServer;
use std::boxed::Box;
use actix::prelude::*;

/// A worker is spun up for each module to ensure that they
/// don't block each other
pub struct ModuleWorker {
    pub pause: PauseBetweenYields,
    pub module: Box<dyn Module>,
    pub status_server_addr: Addr<StatusServer>,
}

impl ModuleWorker {
    pub fn new(wrapper: ModuleWrapper, status_server_addr: Addr<StatusServer>) -> Self {
        let ModuleWrapper (pause, module) = wrapper;
        Self {
            pause,
            module,
            status_server_addr
        }
    }
}
