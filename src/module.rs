/// The core trait of the modular system
pub trait Module: Send + Sync {
    fn yield_next_value(&mut self) -> String;
}

/// A Wrapper which connects the contained module with the duration
/// that should pass in between polls (from the module worker)
pub struct ModuleWrapper(pub Box<dyn Module>, pub crate::PauseBetweenYields);
