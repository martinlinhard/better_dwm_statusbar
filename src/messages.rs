use crate::PauseBetweenYields;
use actix::prelude::*;

/// The actix-message which is sent to the status_server once a worker yields it's value
#[derive(Message)]
#[rtype(result = "()")]
pub struct ModuleMessage(pub String, pub usize);

/// The actix-message which is sent to the workers
/// to signal that a module should be pulled
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClockMessage(pub usize);

/// Message used to initialize the clock-intervals
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddIntervalMessage(pub usize, pub PauseBetweenYields);
