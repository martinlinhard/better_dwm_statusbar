mod bar_modules;
mod clock_server;
mod messages;
mod module;
mod module_worker;
pub mod prelude;
mod status_server;
mod system;
mod x11_utils;

#[macro_use]
extern crate lazy_static;

pub(crate) use system::SYSTEM;

pub type PauseBetweenYields = std::time::Duration;

/// This is the placeholder all format's have to contain!!!
pub const PLACEHOLDER: &'static str = "{VALUE}";

#[macro_export]
macro_rules! start_bar {
    ($config:expr) => {
        use better_dwm_statusbar::prelude::*;
        fn main() {
            System::run(move || {
                let config:Vec<ModuleWrapper> = $config;
                let amount_modules = config.len();

                let (modules, pauses): (Vec<Box<dyn Module>>, Vec<PauseBetweenYields>) = config
                    .into_iter()
                    .fold((Vec::new(), Vec::new()), |mut others, config_item| {
                        let ModuleWrapper(module, pause) = config_item;
                        others.0.push(module);
                        others.1.push(pause);
                        others
                    });

                let status_server = StatusServer::new(amount_modules, " | ").start();
                let mut module_worker = ModuleWorker::new(status_server);

                modules.into_iter().enumerate().for_each(|(index, module)| {
                    module_worker.add_module(index, module);
                });

                let clock_server = ClockServer::new(module_worker.start()).start();

                pauses.into_iter().enumerate().for_each(|(index, pause)| {
                    clock_server.do_send(AddIntervalMessage(index, pause))
                });
            })
            .unwrap();
        }
    };
}
