use crate::module::ModuleWrapper;
use crate::status_server::StatusServer;
use crate::module_worker::ModuleWorker;
use crate::clock_server::ClockServer;
use actix::prelude::*;
use crate::module::Module;
use crate::messages::*;
use crate::PauseBetweenYields;

pub async fn start_bar(config: Vec<ModuleWrapper>, delimiter: &'static str) {
    let amount_modules = config.len();

    // Split the tuples into two vectors
    let (modules, pauses): (Vec<Box<dyn Module>>, Vec<PauseBetweenYields>) =
         config
        .into_iter()
        .fold((Vec::new(), Vec::new()), |mut others, config_item| {
            let ModuleWrapper(module, pause) = config_item;
            others.0.push(module);
            others.1.push(pause);
            others
        });

    let module_worker_addr = SyncArbiter::start(amount_modules, move || {
        let modules = modules.clone();
        let status_server = StatusServer::new(amount_modules, delimiter).start();
        let mut module_worker = ModuleWorker::new(status_server);

        modules
            .into_iter()
            .enumerate()
            .for_each(|(index, module)| {
                module_worker.add_module(index, module);
            });
        module_worker
    });

    let clock_server = ClockServer::new(module_worker_addr).start();

    pauses
        .into_iter()
        .enumerate()
        .for_each(|(index, pause)| clock_server.do_send(AddIntervalMessage(index, pause)));
}
