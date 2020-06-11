use systemstat::{System, Platform};
lazy_static! {
    pub static ref SYSTEM: System = System::new();
}
