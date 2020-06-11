use crate::module::Module;
use systemstat::platform::common::Platform;
use systemstat::saturating_sub_bytes;

pub struct RAMUsage {
    pub format: &'static str,
}

impl RAMUsage {
    pub fn new(format: &'static str) -> Self {
        Self { format }
    }
}

impl Module for RAMUsage {
    fn yield_next_value(&mut self) -> String {
        if let Ok(mem) = crate::SYSTEM.memory() {
            let content = format!(
                "{} / {}",
                saturating_sub_bytes(mem.total, mem.free),
                mem.total
            );
            return self.format.replace(crate::PLACEHOLDER, &content);
        }
        String::new()
    }
}
