pub mod bar_modules;
pub mod module;
pub mod module_worker;
pub mod status_server;
mod x11_utils;
mod system;

pub(crate) use system::SYSTEM;

#[macro_use]
extern crate lazy_static;

pub type PauseBetweenYields = std::time::Duration;

/// This is the placeholder all format's have to contain!!!
pub const PLACEHOLDER: &'static str = "{VALUE}";
