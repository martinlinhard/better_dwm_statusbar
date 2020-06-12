use crate::messages::*;
use crate::module_worker::ModuleWorker;
use actix::prelude::*;

pub struct ClockServer {
    workers: Addr<ModuleWorker>,
}

impl Actor for ClockServer {
    type Context = Context<Self>;
}

impl ClockServer {
    fn new(workers: Addr<ModuleWorker>) -> Self {
        Self { workers }
    }
}

impl Handler<AddIntervalMessage> for ClockServer {
    type Result = ();

    fn handle(&mut self, msg: AddIntervalMessage, ctx: &mut Context<Self>) -> Self::Result {
        let AddIntervalMessage(index, pause) = msg;
        let addr = self.workers.clone();

        ctx.run_interval(pause, move |_, _| {
            addr.do_send(ClockMessage(index.clone()));
        });
    }
}
