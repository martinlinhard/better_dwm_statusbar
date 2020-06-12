mod bar_modules;
mod clock_server;
mod messages;
mod module;
mod module_worker;
mod setup;
mod status_server;
mod system;
mod x11_utils;

#[macro_use]
extern crate lazy_static;

pub(crate) use system::SYSTEM;
pub use crate::setup::start_bar;

pub type PauseBetweenYields = std::time::Duration;

/// This is the placeholder all format's have to contain!!!
pub const PLACEHOLDER: &'static str = "{VALUE}";
