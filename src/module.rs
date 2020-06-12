use actix::prelude::*;
/// The core trait of the modular system
pub trait Module: Send + Sync {
    fn yield_next_value(&mut self) -> String;
}

/// A Wrapper which connects the contained module with the duration
/// that should pass in between polls (from the module worker)
pub struct ModuleWrapper(pub Box<dyn Module>, pub crate::PauseBetweenYields);

/// The actix-message which is sent to the status_server once a worker yields it's value
#[derive(Message)]
#[rtype(result = "()")]
pub struct ModuleMessage(pub String, pub usize);
